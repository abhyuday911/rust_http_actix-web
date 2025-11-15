use std::sync::Arc;

use crate::{Order, OrderAction, OrderBook};
use serde_json::to_string_pretty;
use tokio::sync::{Mutex, mpsc};

pub async fn run_engine(mut receiver: mpsc::Receiver<Order>, order_book: Arc<Mutex<OrderBook>>) {
    while let Some(order) = receiver.recv().await {
        let mut book = order_book.lock().await;
        book.next_order_id += 1;

        let book_type = match order.order_action {
            OrderAction::Buy => &mut book.bids,
            OrderAction::Sell => &mut book.asks,
        };

        book_type
            .entry(order.price)
            .or_insert_with(Vec::new)
            .push(order.clone());

        println!("{}", to_string_pretty(book_type).unwrap());
    }
}
