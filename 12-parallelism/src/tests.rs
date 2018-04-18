#![allow(unused_mut, unused_variables, dead_code)]

mod simple_thread {

    use std::thread;

    #[test]
    fn spawn_a_thread_and_get_his_id() {
        let father_id = thread::current().id();
        let child = thread::spawn(|| 1 + 1);
        let child_id = child.thread().id();
        assert_ne!(father_id, child_id);
    }

    use std::{
        sync::mpsc::channel,
        time::Duration,
    };

    #[test]
    fn spawn_a_thread_and_use_rust_channel_to_communicate_between_threads() {
        let message_to_send = "hello";
        let (sender, receiver) = channel();

        thread::spawn(move || {
            sender.send(message_to_send).unwrap();
        });

        let msg_recv = receiver.recv_timeout(Duration::from_millis(20))
                                    .unwrap_or("nope");

        assert_eq!(message_to_send, msg_recv);
    }

    #[test]
    fn create_multiple_producers_by_cloning_the_transmitter() {
        let (sender, receiver) = channel();
        let other_sender = sender.clone() ;

        thread::spawn(move || {
            sender.send(1).unwrap();
        });

        thread::spawn(move || {
            other_sender.send(2).unwrap();
        });

        let mut nb_msg = 0;
        while let Ok(msg) = receiver.recv_timeout(Duration::from_millis(20)) {
            nb_msg+= 1;
        }

        assert_eq!(2, nb_msg);
    }
}
