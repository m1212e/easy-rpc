#[macro_export]
macro_rules! unwrap_oneshot {
    ($expression:expr, $sender:expr, $reciever:expr) => {{

        match $expression {
            Ok(val) => val,
            Err(err) => {
                $sender.send(Err(err.into())).unwrap();
                return $reciever;
            }
        }
    }};
}
