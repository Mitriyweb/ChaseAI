use app::network::interface_detector::{InterfaceDetector, InterfaceType};

#[test]
fn test_detection_not_empty() {
    let interfaces = InterfaceDetector::detect_all().unwrap();
    assert!(!interfaces.is_empty());
}

#[test]
fn test_loopback_present() {
    let interfaces = InterfaceDetector::detect_loopback().unwrap();
    assert!(!interfaces.is_empty());
    assert!(interfaces
        .iter()
        .all(|i| i.interface_type == InterfaceType::Loopback));
}
