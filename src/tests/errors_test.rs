#[cfg(test)]
mod tests {
    use crate::errors::ErrorArray;
    use crate::errors::ErrorArrayItem;
    use crate::errors::Errors;
    use crate::errors::WarningArray;
    use crate::errors::WarningArrayItem;
    use crate::errors::Warnings;

    // use super::*;
    use std::io;
    use std::net;
    use std::net::AddrParseError;
    use std::sync::mpsc;
    // use std::time::SystemTime;

    #[test]
    fn test_error_array_item_creation() {
        let error_item = ErrorArrayItem::new(Errors::OpeningFile, String::from("Failed to open file"));
        assert_eq!(error_item.err_type, Errors::OpeningFile);
        assert_eq!(error_item.err_mesg, "Failed to open file");
    }

    #[test]
    fn test_warning_array_item_creation() {
        let warning_item = WarningArrayItem::new(Warnings::Warning);
        assert_eq!(warning_item.warn_type, Warnings::Warning);
        assert!(warning_item.warn_mesg.is_none());

        let detailed_warning_item = WarningArrayItem::new_details(
            Warnings::OutdatedVersion,
            String::from("Version is outdated")
        );
        assert_eq!(detailed_warning_item.warn_type, Warnings::OutdatedVersion);
        assert_eq!(detailed_warning_item.warn_mesg.as_deref(), Some("Version is outdated"));
    }

    #[test]
    fn test_error_array_operations() {
        let mut error_array = ErrorArray::new_container();
        let error_item1 = ErrorArrayItem::new(Errors::ReadingFile, String::from("Failed to read file"));
        let error_item2 = ErrorArrayItem::new(Errors::CreatingFile, String::from("Failed to create file"));

        error_array.push(error_item1);
        error_array.push(error_item2);

        assert_eq!(error_array.len(), 2);
        
        // Displaying and clearing the array
        error_array.clone().display(false);
        assert_eq!(error_array.len(), 0);
    }

    #[test]
    fn test_warning_array_operations() {
        let mut warning_array = WarningArray::new_container();
        let warning_item1 = WarningArrayItem::new(Warnings::UnexpectedBehavior);
        let warning_item2 = WarningArrayItem::new_details(Warnings::ConnectionLost, String::from("Connection lost"));

        warning_array.push(warning_item1);
        warning_array.push(warning_item2);

        assert_eq!(warning_array.len(), 2);
        
        // Displaying and clearing the array
        warning_array.clone().display();
        assert_eq!(warning_array.len(), 0);
    }

    #[test]
    fn test_error_array_conversion() {
        // Converting io::Error
        let io_error = io::Error::new(io::ErrorKind::Other, "I/O error");
        let error_item: ErrorArrayItem = io_error.into();
        assert_eq!(error_item.err_type, Errors::InputOutput);
        assert_eq!(error_item.err_mesg, "I/O error");

        // Converting net::AddrParseError
        let addr_error: AddrParseError = "invalid address".parse::<net::IpAddr>().unwrap_err();
        let error_item: ErrorArrayItem = addr_error.into();
        assert_eq!(error_item.err_type, Errors::InputOutput);
        assert_eq!(error_item.err_mesg, "invalid IP address syntax");

        // Converting mpsc::SendError
        let (sender, receiver) = mpsc::channel::<i32>();
        drop(receiver);
        let send_error: mpsc::SendError<i32> = sender.send(1).unwrap_err();
        let error_item: ErrorArrayItem = send_error.into();
        assert_eq!(error_item.err_type, Errors::InputOutput);
        assert_eq!(error_item.err_mesg, "sending on a closed channel");

        // // Converting SystemTimeError
        // let system_time_error: SystemTime = SystemTime::now() - SystemTime::UNIX_EPOCH;
        // let error_item: ErrorArrayItem = system_time_error.elapsed().unwrap_err().into();
        // assert_eq!(error_item.err_type, Errors::InputOutput);
        // assert_eq!(error_item.err_mesg, "second time provided was later than self");

        // Ensure that we can push these items to an ErrorArray
        let mut error_array = ErrorArray::new_container();
        error_array.push(error_item);
        assert_eq!(error_array.len(), 1);
    }

    #[test]
    fn test_warning_array_append() {
        let warning_item1 = WarningArrayItem::new(Warnings::UnexpectedBehavior);
        let warning_item2 = WarningArrayItem::new_details(Warnings::ConnectionLost, String::from("Connection lost"));
        
        let mut array1 = WarningArray::new(vec![warning_item1.clone()]);
        let array2 = WarningArray::new(vec![warning_item2.clone()]);

        array1.append(array2);
        assert_eq!(array1.len(), 2);
        let vec = array1.0.read().unwrap();
        assert_eq!(vec[0].warn_type, Warnings::UnexpectedBehavior);
        assert_eq!(vec[1].warn_type, Warnings::ConnectionLost);
    }

    #[test]
    fn test_error_array_append() {
        let error_item1 = ErrorArrayItem::new(Errors::ReadingFile, String::from("Failed to read file"));
        let error_item2 = ErrorArrayItem::new(Errors::CreatingFile, String::from("Failed to create file"));
        
        let mut array1 = ErrorArray::new(vec![error_item1.clone()]);
        let array2 = ErrorArray::new(vec![error_item2.clone()]);

        array1.append(array2);
        assert_eq!(array1.len(), 2);
        let vec = array1.0.read().unwrap();
        assert_eq!(vec[0].err_type, Errors::ReadingFile);
        assert_eq!(vec[1].err_type, Errors::CreatingFile);
    }
}
