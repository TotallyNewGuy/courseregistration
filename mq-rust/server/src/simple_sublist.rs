use crate::client::ClientMessageSender;
use crate::error::{NError, Result, ERROR_SUBSCRIBTION_NOT_FOUND};
use bitflags::_core::cmp::Ordering;
use std::collections::{BTreeSet, HashMap};
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Debug)]
pub struct Subscription {
    pub msg_sender: Arc<Mutex<ClientMessageSender>>,
    pub subject: String,
    pub queue: Option<String>,
    pub sid: String,
}
impl Subscription {
    pub fn new(
        subject: &str,
        queue: Option<&str>,
        sid: &str,
        msg_sender: Arc<Mutex<ClientMessageSender>>,
    ) -> Self {
        Self {
            subject: subject.to_string(),
            queue: queue.map(|s| s.to_string()),
            sid: sid.to_string(),
            msg_sender,
        }
    }
}
#[derive(Debug, Default)]
pub struct SubResult {
    pub subs: Vec<ArcSubscription>,
    pub qsubs: Vec<Vec<ArcSubscription>>,
}
impl SubResult {
    pub fn is_empty(&self) -> bool {
        self.subs.len() == 0 && self.qsubs.len() == 0
    }
}
pub type ArcSubscription = Arc<Subscription>;
/*
因为孤儿原则,所以必须单独定义ArcSubscription
*/
#[derive(Debug, Clone)]
pub struct ArcSubscriptionWrapper(ArcSubscription);
impl std::cmp::PartialEq for ArcSubscriptionWrapper {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}
/*
为了能够将ArcSubscription,必须实现下面这些Trait

*/
impl std::cmp::Eq for ArcSubscriptionWrapper {}
impl std::cmp::PartialOrd for ArcSubscriptionWrapper {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::Ord for ArcSubscriptionWrapper {
    fn cmp(&self, other: &Self) -> Ordering {
        let a = self.0.as_ref() as *const Subscription as usize;
        let b = other.0.as_ref() as *const Subscription as usize;
        a.cmp(&b)
    }
}
pub type ArcSubResult = Arc<SubResult>;
pub trait SubListTrait {
    fn insert(&mut self, sub: Arc<Subscription>) -> Result<()>;
    fn remove(&mut self, sub: Arc<Subscription>) -> Result<()>;
    fn match_subject(&mut self, subject: &str) -> Result<ArcSubResult>;
}
#[derive(Debug, Default)]
pub struct SimpleSubList {
    subs: HashMap<String, BTreeSet<ArcSubscriptionWrapper>>,
    qsubs: HashMap<String, HashMap<String, BTreeSet<ArcSubscriptionWrapper>>>,
}

impl SubListTrait for SimpleSubList {
    fn insert(&mut self, sub: Arc<Subscription>) -> Result<()> {
        if let Some(ref q) = sub.queue {
            let entry = self
                .qsubs
                .entry(sub.subject.clone())
                .or_insert(Default::default());
            let queue = entry.entry(q.clone()).or_insert(Default::default());
            queue.insert(ArcSubscriptionWrapper(sub));
        } else {
            let subs = self
                .subs
                .entry(sub.subject.clone())
                .or_insert(Default::default());
            subs.insert(ArcSubscriptionWrapper(sub));
        }
        Ok(())
    }

    fn remove(&mut self, sub: Arc<Subscription>) -> Result<()> {
        if let Some(ref q) = sub.queue {
            if let Some(subs) = self.qsubs.get_mut(&sub.subject) {
                if let Some(qsubs) = subs.get_mut(q) {
                    qsubs.remove(&ArcSubscriptionWrapper(sub.clone()));
                    if qsubs.is_empty() {
                        subs.remove(q);
                    }
                } else {
                    return Err(NError::new(ERROR_SUBSCRIBTION_NOT_FOUND));
                }
                if subs.is_empty() {
                    self.qsubs.remove(&sub.subject);
                }
            } else {
                return Err(NError::new(ERROR_SUBSCRIBTION_NOT_FOUND));
            }
        } else {
            if let Some(subs) = self.subs.get_mut(&sub.subject) {
                subs.remove(&ArcSubscriptionWrapper(sub.clone()));
                if subs.is_empty() {
                    self.subs.remove(&sub.subject);
                }
            }
        }
        Ok(())
    }

    fn match_subject(&mut self, subject: &str) -> Result<ArcSubResult> {
        let mut r = SubResult::default();
        if let Some(subs) = self.subs.get(subject) {
            for s in subs {
                r.subs.push(s.0.clone());
            }
        }
        if let Some(qsubs) = self.qsubs.get(subject) {
            for (_, qsub) in qsubs {
                let mut v = Vec::with_capacity(qsub.len());
                for s in qsub {
                    v.push(s.0.clone());
                }
                r.qsubs.push(v);
            }
        }
        Ok(Arc::new(r))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::client::new_test_tcp_writer;

    #[tokio::main]
    #[test]
    async fn test_match() {
        let mut sl = SimpleSubList::default();
        let mut subs = Vec::new();
        let r = sl.match_subject("test").unwrap();
        assert_eq!(r.subs.len(), 0);
        assert_eq!(r.qsubs.len(), 0);
        let sub = Arc::new(Subscription::new("test", None, "1", new_test_tcp_writer()));
        subs.push(sub.clone());
        let r = sl.insert(sub);
        assert!(!r.is_err());
        let r = sl.match_subject("test").unwrap();
        assert_eq!(r.subs.len(), 1);
        assert_eq!(r.qsubs.len(), 0);
        let sub = Arc::new(Subscription::new("test", None, "1", new_test_tcp_writer()));
        subs.push(sub.clone());
        let r = sl.insert(sub);
        assert!(!r.is_err());
        let r = sl.match_subject("test").unwrap();
        assert_eq!(r.subs.len(), 2);
        assert_eq!(r.qsubs.len(), 0);
        let sub = Arc::new(Subscription::new(
            "test",
            Some("q"),
            "1",
            new_test_tcp_writer(),
        ));
        subs.push(sub.clone());
        let r = sl.insert(sub);
        assert!(!r.is_err());
        let r = sl.match_subject("test").unwrap();
        assert_eq!(r.subs.len(), 2);
        assert_eq!(r.qsubs.len(), 1);
        let sub = Arc::new(Subscription::new(
            "test",
            Some("q"),
            "1",
            new_test_tcp_writer(),
        ));
        subs.push(sub.clone());
        let r = sl.insert(sub);
        assert!(!r.is_err());
        let r = sl.match_subject("test").unwrap();
        assert_eq!(r.subs.len(), 2);
        assert_eq!(r.qsubs.len(), 1);

        let sub = Arc::new(Subscription::new(
            "test",
            Some("q2"),
            "1",
            new_test_tcp_writer(),
        ));
        subs.push(sub.clone());
        let r = sl.insert(sub);
        assert!(!r.is_err());
        let r = sl.match_subject("test").unwrap();
        assert_eq!(r.subs.len(), 2);
        assert_eq!(r.qsubs.len(), 2);

        let s = subs.pop().unwrap();
        let r = sl.remove(s);
        assert!(!r.is_err());
        let r = sl.match_subject("test").unwrap();
        assert_eq!(r.subs.len(), 2);
        assert_eq!(r.qsubs.len(), 1);

        let s = subs.pop().unwrap();
        let r = sl.remove(s);
        assert!(!r.is_err());
        let r = sl.match_subject("test").unwrap();
        assert_eq!(r.subs.len(), 2);
        assert_eq!(r.qsubs.len(), 1);

        let s = subs.pop().unwrap();
        let r = sl.remove(s);
        assert!(!r.is_err());
        let r = sl.match_subject("test").unwrap();
        assert_eq!(r.subs.len(), 2);
        assert_eq!(r.qsubs.len(), 0);

        let s = subs.pop().unwrap();
        let r = sl.remove(s);
        assert!(!r.is_err());
        let r = sl.match_subject("test").unwrap();
        assert_eq!(r.subs.len(), 1);
        assert_eq!(r.qsubs.len(), 0);

        let s = subs.pop().unwrap();
        let r = sl.remove(s);
        assert!(!r.is_err());
        let r = sl.match_subject("test").unwrap();
        assert_eq!(r.subs.len(), 0);
        assert_eq!(r.qsubs.len(), 0);
    }
}