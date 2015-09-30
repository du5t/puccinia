#[cfg(test)]
mod tests {
    use super::controlsocket::controlport;

    // these two tests actually exercise the constructor and the three methods
    fn constructor_no_connection() {
        let testIP = "127.0.0.1";
        let testPort = 9051;
        let testControlPort = controlport::ControlPort::new(testIP, testPort,
                                                            false);
        assert_eq!(testControlPort.address.to_string(), testIP);
    }

    fn constructor_connection() {
        let testIP = "127.0.0.1";
        let testPort = 9051;
        let testControlPort = controlport::ControlPort::new(testIP, testPort,
                                                            true);
        assert_eq!(testControlPort.stream.peer_addr(), testIP);
    }

    #[should_panic(expected = "Could not connect with given addresses:")]
    fn constructor_bad_addr() {
        let testIP = "9999999";
        let testPort = 9051;
        let testControlPort = controlport::ControlPort::new(testIP, testPort,
                                                            true);
        assert_eq!(testControlPort.stream.peer_addr(), testIP);
    }
}

