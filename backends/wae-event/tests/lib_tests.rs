use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};
use wae_event::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct TestEvent {
    id: String,
    message: String,
}

impl Event for TestEvent {
    fn event_type(&self) -> EventTypeName {
        "test.event".to_string()
    }

    fn event_id(&self) -> &EventId {
        &self.id
    }

    fn timestamp(&self) -> u64 {
        SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as u64
    }
}

#[tokio::test]
async fn test_event_data() {
    let event = TestEvent { id: "test-1".to_string(), message: "Hello".to_string() };

    let data = EventData::new(&event).unwrap();
    assert_eq!(data.event_type(), "test.event");
    assert_eq!(data.id(), "test-1");
}

#[tokio::test]
async fn test_in_memory_store() {
    let store = InMemoryEventStore::new();
    let event = TestEvent { id: "test-1".to_string(), message: "Hello".to_string() };

    let data = EventData::new(&event).unwrap();
    store.append(&data).await.unwrap();

    let count = store.count().await.unwrap();
    assert_eq!(count, 1);

    let events = store.get_events("test.event").await.unwrap();
    assert_eq!(events.len(), 1);
}

#[tokio::test]
async fn test_event_bus_subscribe() {
    let config = EventBusConfig::default();
    let bus = EventBus::new(config);

    let handler =
        AsyncEventHandler::new(vec!["test.event".to_string()], move |_event: EventData| Box::pin(async move { Ok(()) }));

    let sub = bus.subscribe(vec!["test.event".to_string()], handler).unwrap();
    assert_eq!(bus.subscription_count(), 1);

    bus.unsubscribe(&sub.id).unwrap();
    assert_eq!(bus.subscription_count(), 0);
}

#[tokio::test]
async fn test_base_event() {
    let event = BaseEvent::new("user.created", "user-123".to_string());
    assert_eq!(event.event_type(), "user.created");
    assert!(!event.event_id().is_empty());
}
