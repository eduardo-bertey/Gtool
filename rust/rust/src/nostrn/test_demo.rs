/// Demo ejecutable del cliente NostrDM síncrono
/// Usa las credenciales y relays proporcionados para probar la funcionalidad

mod client;
mod keys;
mod relays;

use client::NostrClient;
use anyhow::Result;
use std::time::Duration;
use nostr_sdk::ToBech32;

fn main() -> Result<()> {
    println!("=== Demo NostrDM Síncrono ===\n");

    // Datos de prueba
    let nsec = "nsec1cg99f0q7ptvn6w0r2hfns848ses28jqnp40vs4v0s7e67fg88u0sdvqljj";
    let npub = "npub1x20uef6xzepy5ygzp92trw27prdy7v3gqxqeed24tlmr5cht5gdsazrq2v";
    
    // Relays
    let dm_relays = vec!["wss://nos.lol".to_string()];
    let read_relays = vec!["wss://relay.primal.net".to_string()];
    
    // PASO 1: Inicializar cliente
    println!("1️⃣  Inicializando cliente...");
    let mut client = NostrClient::new(Some(nsec), npub)?;
    println!("   ✓ Cliente inicializado");
    println!("   📍 Tu npub: {}", client.get_public_key()?);
    println!("   📍 Destinatario: {}\n", client.get_peer_public_key()?);

    // PASO 2: Configurar relays
    println!("2️⃣  Configurando relays...");
    println!("   📡 DM relay: wss://nos.lol");
    println!("   📡 Read relay: wss://relay.primal.net");
    println!("   📡 Write relays: incluye fallbacks públicos");
    
    client.add_relays(dm_relays, Some(read_relays))?;
    println!("   ✓ Relays configurados y conectados\n");

    // PASO 3: Suscribirse
    println!("3️⃣  Suscribiéndose a mensajes...");
    client.subscribe()?;
    println!("   ✓ Suscripción activa\n");

    // PASO 4: Revisar mensajes históricos primero
    println!("4️⃣  Revisando mensajes históricos...");
    let messages = client.poll_messages()?;
    
    if messages.is_empty() {
        println!("   ℹ️  No hay mensajes históricos\n");
    } else {
        println!("   ✉️  {} mensaje(s) histórico(s):\n", messages.len());
        for (i, msg) in messages.iter().enumerate() {
            println!("   📨 Mensaje #{}", i + 1);
            println!("      De: {}", msg.sender.to_bech32()?);
            println!("      Contenido: {}", msg.content);
            println!("      Timestamp: {}\n", msg.timestamp);
        }
    }

    // PASO 5: Enviar mensaje de prueba
    println!("5️⃣  Enviando mensaje de prueba...");
    let test_message = "🧪 Test desde NostrDM Sync - Cliente sin threads ni channels!";
    client.send_message(test_message)?;
    println!("   ✓ Mensaje enviado: {}\n", test_message);

    // PASO 6: Esperar un poco para que lleguen respuestas
    println!("6️⃣  Esperando 5 segundos para posibles respuestas...");
    std::thread::sleep(Duration::from_secs(5));

    // PASO 7: Revisar mensajes nuevos
    println!("7️⃣  Revisando mensajes nuevos...");
    let new_messages = client.poll_messages()?;
    
    if new_messages.is_empty() {
        println!("   ℹ️  No hay mensajes nuevos\n");
    } else {
        println!("   ✉️  {} mensaje(s) nuevo(s):\n", new_messages.len());
        for (i, msg) in new_messages.iter().enumerate() {
            println!("   📨 Mensaje #{}", i + 1);
            println!("      De: {}", msg.sender.to_bech32()?);
            println!("      Contenido: {}", msg.content);
            println!("      Timestamp: {}\n", msg.timestamp);
        }
    }

    // PASO 8: Demostrar uso manual
    println!("7️⃣  Demostrando uso manual de funciones:\n");
    
    println!("   📤 Enviando segundo mensaje...");
    client.send_message("Segundo mensaje de prueba")?;
    println!("      ✓ Enviado\n");

    println!("   📥 Revisando mensajes nuevamente...");
    let more_messages = client.poll_messages()?;
    println!("      {} mensaje(s) nuevo(s)\n", more_messages.len());

    // PASO 8: Mostrar características
    println!("✅ Demo completado exitosamente!\n");
    println!("=== Características demostradas ===");
    println!("✓ Cliente inicializado UNA sola vez");
    println!("✓ Relays conectados UNA sola vez");
    println!("✓ Sin threads ni channels");
    println!("✓ API completamente síncrona");
    println!("✓ send_message() - llamado manualmente");
    println!("✓ poll_messages() - llamado manualmente");
    println!("✓ Configuración reutilizable");

    Ok(())
}
