/// Ejemplo de uso del cliente NostrDM síncrono
/// 
/// Este ejemplo muestra cómo usar NostrClient para:
/// 1. Inicializar el cliente una sola vez
/// 2. Configurar relays
/// 3. Enviar mensajes
/// 4. Recibir mensajes mediante polling
/// 
/// Nota: Este es un ejemplo de referencia. Para usarlo, debes:
/// - Reemplazar "nsec1..." con tu clave privada real
/// - Reemplazar "npub1..." con la clave pública del destinatario
/// - Reemplazar los relays con relays reales

use anyhow::Result;

// Descomenta estas líneas si quieres compilar este ejemplo como binario
// mod client;
// mod keys;
// mod relays;
// use client::NostrClient;

fn main() -> Result<()> {
    println!("=== Ejemplo de NostrClient Síncrono ===\n");

    // PASO 1: Inicializar el cliente UNA SOLA VEZ
    // Esto crea el cliente y guarda la configuración
    println!("1. Inicializando cliente...");
    
    // Datos de prueba
    let mut client = NostrClient::new(
        Some("nsec1cg99f0q7ptvn6w0r2hfns848ses28jqnp40vs4v0s7e67fg88u0sdvqljj"),
        "npub1x20uef6xzepy5ygzp92trw27prdy7v3gqxqeed24tlmr5cht5gdsazrq2v"
    )?;
    
    println!("   ✓ Cliente inicializado");
    println!("   Tu npub: {}", client.get_public_key()?);
    println!("   Destinatario: {}\n", client.get_peer_public_key()?);

    // PASO 2: Configurar relays
    // Esto se hace UNA SOLA VEZ, no se reconecta en cada operación
    println!("2. Configurando relays...");
    
    let dm_relays = vec![
        "wss://nos.lol".to_string(),
    ];
    
    let read_relays = vec![
        "wss://relay.primal.net".to_string(),
    ];
    
    client.add_relays(dm_relays, Some(read_relays))?;
    println!("   ✓ Relays configurados y conectados\n");

    // PASO 3: Suscribirse a mensajes
    println!("3. Suscribiéndose a mensajes...");
    client.subscribe()?;
    println!("   ✓ Suscripción activa\n");

    // ========================================
    // USO MANUAL: Puedes llamar estas funciones cuando quieras
    // ========================================
    
    // FUNCIÓN 1: send_message() - Enviar mensaje cuando TÚ quieras
    println!("📤 Enviando mensaje manualmente...");
    client.send_message("¡Hola desde NostrClient síncrono!")?;
    println!("   ✓ Mensaje enviado\n");

    // Puedes enviar más mensajes cuando quieras
    println!("📤 Enviando otro mensaje...");
    client.send_message("Este es otro mensaje de prueba")?;
    println!("   ✓ Mensaje enviado\n");

    // FUNCIÓN 2: poll_messages() - Revisar mensajes cuando TÚ quieras
    println!("📥 Revisando mensajes manualmente...");
    let messages = client.poll_messages()?;
    
    if messages.is_empty() {
        println!("   No hay mensajes nuevos\n");
    } else {
        println!("   {} mensaje(s) recibido(s):", messages.len());
        for msg in messages {
            println!("   📨 De: {}", msg.sender.to_bech32()?);
            println!("      Contenido: {}", msg.content);
            println!("      Timestamp: {}\n", msg.timestamp);
        }
    }

    // Puedes revisar mensajes nuevamente cuando quieras
    println!("📥 Revisando mensajes otra vez...");
    let more_messages = client.poll_messages()?;
    println!("   {} mensaje(s) nuevo(s)\n", more_messages.len());

    // ========================================
    // OPCIONAL: Loop automático para monitoreo continuo
    // ========================================
    // Si quieres monitorear mensajes continuamente, puedes usar un loop
    // Pero NO es necesario - puedes llamar poll_messages() manualmente cuando quieras
    println!("🔄 Iniciando loop de monitoreo continuo (presiona Ctrl+C para salir)...\n");
    
    loop {
        // Poll de mensajes sin bloquear
        let messages = client.poll_messages()?;
        
        if !messages.is_empty() {
            for msg in messages {
                println!("📨 Mensaje recibido:");
                println!("   De: {}", msg.sender.to_bech32()?);
                println!("   Contenido: {}", msg.content);
                println!("   Timestamp: {}\n", msg.timestamp);
            }
        }
        
        // Esperar un poco antes del siguiente poll
        std::thread::sleep(std::time::Duration::from_secs(2));
    }
   

    println!("\n=== Características clave ===");
    println!("✓ Sin threads ni channels");
    println!("✓ Cliente se inicializa UNA sola vez");
    println!("✓ Relays se conectan UNA sola vez");
    println!("✓ API completamente síncrona");
    println!("✓ Configuración almacenada y reutilizable");

    Ok(())
}
