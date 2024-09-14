//! This module is written for Rinf demonstrations.

use crate::messages::*;

async fn communicate() {
    // Send signals to Dart like below.
    SmallNumber { number: 7 }.send_signal_to_dart();

    // Get receivers that listen to Dart signals like below.
    let receiver = SmallText::get_dart_signal_receiver();
    while let Some(dart_signal) = receiver.recv().await {
        let message: SmallText = dart_signal.message;
        rinf::debug_print!("{message:?}");
    }
}
