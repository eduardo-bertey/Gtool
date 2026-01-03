use clap::Parser;
use nostr_sdk::prelude::*;
use std::str::FromStr;
use tokio::io::AsyncBufReadExt;

// Relay URL
const RELAY_URL: &str = "wss://relay.mostro.network";
const POW_DIFFICULTY: u8 = 2;
// We set N in seconds (600 seconds = 10 minutes)
const N_SECONDS: u64 = 600;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Sender's private key (hex or bech32)
    #[arg(short = 's', long = "sender-secret", requires = "receiver_pubkey")]
    sender_secret: Option<String>,

    /// Receiver's public key (hex or bech32)
    #[arg(short = 'p', long = "receiver-pubkey", requires = "sender_secret")]
    receiver_pubkey: Option<String>,

    /// Shared secret key (hex)
    #[arg(short = 'k', long = "shared-key", conflicts_with_all = ["sender_secret", "receiver_pubkey"])]
    shared_key: Option<String>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let shared_keys: Keys;
    let sender_keys: Option<Keys>;
    let is_observer = args.shared_key.is_some();

    if let Some(shared_key_hex) = args.shared_key {
        let shared_secret_key = SecretKey::from_str(&shared_key_hex).expect("Invalid shared key");
        shared_keys = Keys::new(shared_secret_key);
        sender_keys = None;
    } else {
        let sender_secret = args.sender_secret.expect("Sender secret is required");
        let receiver_pubkey_str = args.receiver_pubkey.expect("Receiver pubkey is required");

        let sk = Keys::parse(&sender_secret).expect("Invalid sender's private key");
        sender_keys = Some(sk.clone());
        let receiver_pubkey = PublicKey::from_str(&receiver_pubkey_str).expect("Invalid recipient public key");

        let shared_key = nostr_sdk::util::generate_shared_key(
            sk.secret_key(),
            &receiver_pubkey,
        ).expect("Error generating shared key");
        let shared_secret_key = SecretKey::from_slice(&shared_key).unwrap();
        shared_keys = Keys::new(shared_secret_key);
    }

    println!("Shared Key: {}", shared_keys.secret_key().to_secret_hex());
    if is_observer {
        println!("Mode: Observer");
    } else {
        println!("Mode: Participant");
    }

    // Client setup
    let client = Client::new(Keys::generate());
    client.add_relay(RELAY_URL).await?;
    client.connect().await;

    let filter = Filter::new()
        .kind(Kind::GiftWrap)
        .pubkey(shared_keys.public_key());
    client.subscribe(filter, None).await?;

    println!("Listening for messages...");
    
    let mut stdin = tokio::io::BufReader::new(tokio::io::stdin()).lines();
    let mut notifications = client.notifications();
    
    loop {
        tokio::select! {
            Ok(Some(line)) = stdin.next_line() => {
                let message = line.trim();
                if !message.is_empty() {
                    if let Some(sender) = &sender_keys {
                         if let Err(e) = send_message(&client, sender, shared_keys.public_key(), message).await {
                             eprintln!("Error sending message: {}", e);
                         }
                    }
                }
            }
            Ok(notification) = notifications.recv() => {
                if let RelayPoolNotification::Event { event, .. } = notification {
                     if let Ok(inner_event) = mostro_unwrap(&shared_keys, *event).await {
                        // Filter messages from the last N seconds
                        let now = Timestamp::now().as_u64();
                        let msg_time = inner_event.created_at.as_u64();
                        if now.saturating_sub(msg_time) > N_SECONDS {
                            continue;
                        }

                        let message = inner_event.content;
                        let pubkey = inner_event.pubkey;
                        
                        if let Some(sender) = &sender_keys {
                            if sender.public_key() == pubkey {
                                println!("You: {}", message);
                            } else {
                                println!("{}: {}", pubkey, message);
                            }
                        } else {
                             println!("{}: {}", pubkey, message);
                        }
                     }
                }
            }
            else => break,
        }
    }

    Ok(())
}

async fn send_message(
    client: &Client,
    sender: &Keys,
    receiver: PublicKey,
    message: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let wrapped_event = mostro_wrap(sender, receiver, message, vec![]).await?;
    client.send_event(&wrapped_event).await?;
    Ok(())
}

pub async fn mostro_wrap(
    sender: &Keys,
    receiver: PublicKey,
    message: &str,
    extra_tags: Vec<Tag>,
) -> Result<Event, Box<dyn std::error::Error>> {
    let inner_event = EventBuilder::text_note(message)
        .build(sender.public_key())
        .sign(sender)
        .await?;
    
    let keys: Keys = Keys::generate();
    let encrypted_content: String = nip44::encrypt(
        keys.secret_key(),
        &receiver,
        inner_event.as_json(),
        nip44::Version::V2,
    )?;

    let mut tags = vec![Tag::public_key(receiver)];
    tags.extend(extra_tags);

    let wrapped_event = EventBuilder::new(Kind::GiftWrap, encrypted_content)
        .pow(POW_DIFFICULTY)
        .tags(tags)
        .custom_created_at(Timestamp::tweaked(nip59::RANGE_RANDOM_TIMESTAMP_TWEAK))
        .sign_with_keys(&keys)?;
    Ok(wrapped_event)
}

pub async fn mostro_unwrap(
    receiver: &Keys,
    event: Event,
) -> Result<Event, Box<dyn std::error::Error>> {
    let decrypted_content = nip44::decrypt(receiver.secret_key(), &event.pubkey, &event.content)?;
    let inner_event = Event::from_json(&decrypted_content)?;
    inner_event.verify()?;
    Ok(inner_event)
}
