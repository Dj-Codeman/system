#[cfg(test)]
mod tests {
    use crate::errors::ErrorArray;
    use crate::errors::ErrorArrayItem;
    use crate::errors::Errors;
    use crate::errors::OkWarning;
    use crate::errors::UnifiedResult;
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
        let error_item =
            ErrorArrayItem::new(Errors::OpeningFile, String::from("Failed to open file"));
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
            String::from("Version is outdated"),
        );
        assert_eq!(detailed_warning_item.warn_type, Warnings::OutdatedVersion);
        assert_eq!(
            detailed_warning_item.warn_mesg.as_deref(),
            Some("Version is outdated")
        );
    }

    #[test]
    fn test_error_array_operations() {
        let mut error_array = ErrorArray::new_container();
        let error_item1 =
            ErrorArrayItem::new(Errors::ReadingFile, String::from("Failed to read file"));
        let error_item2 =
            ErrorArrayItem::new(Errors::CreatingFile, String::from("Failed to create file"));

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
        let warning_item2 = WarningArrayItem::new_details(
            Warnings::ConnectionLost,
            String::from("Connection lost"),
        );

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
        let warning_item2 = WarningArrayItem::new_details(
            Warnings::ConnectionLost,
            String::from("Connection lost"),
        );

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
        let error_item1 =
            ErrorArrayItem::new(Errors::ReadingFile, String::from("Failed to read file"));
        let error_item2 =
            ErrorArrayItem::new(Errors::CreatingFile, String::from("Failed to create file"));

        let mut array1 = ErrorArray::new(vec![error_item1.clone()]);
        let array2 = ErrorArray::new(vec![error_item2.clone()]);

        array1.append(array2);
        assert_eq!(array1.len(), 2);
        let vec = array1.0.read().unwrap();
        assert_eq!(vec[0].err_type, Errors::ReadingFile);
        assert_eq!(vec[1].err_type, Errors::CreatingFile);
    }

    #[test]
    fn test_warning_array() {
        let mut warning_array = WarningArray::new_container();
        let warning_item = WarningArrayItem::new(Warnings::Warning);
        warning_array.push(warning_item);

        assert_eq!(warning_array.len(), 1);
        let warnings = warning_array.0.read().unwrap();
        assert_eq!(warnings[0].warn_type, Warnings::Warning);
    }

    #[test]
    fn test_error_array() {
        let mut error_array = ErrorArray::new_container();
        let error_item =
            ErrorArrayItem::new(Errors::OpeningFile, String::from("Failed to open file"));
        error_array.push(error_item);

        assert_eq!(error_array.len(), 1);
        let errors = error_array.0.read().unwrap();
        assert_eq!(errors[0].err_type, Errors::OpeningFile);
        assert_eq!(errors[0].err_mesg, "Failed to open file");
    }

    #[test]
    fn test_unified_result_ok() {
        let warning_item = WarningArrayItem::new(Warnings::Warning);
        let warning_array = WarningArray::new(vec![warning_item]);
        let ok_warning = OkWarning {
            data: 42,
            warning: warning_array.clone(),
        };

        let result = UnifiedResult::new_warn(Ok(ok_warning));
        assert!(result.is_ok());

        match result {
            UnifiedResult::ResultWarning(Ok(ok_warning)) => {
                assert_eq!(ok_warning.data, 42);
                assert_eq!(ok_warning.warning.len(), 1);
            }
            _ => panic!("Expected ResultWarning with Ok"),
        }
    }

    #[test]
    fn test_unified_result_err() {
        let error_item =
            ErrorArrayItem::new(Errors::OpeningFile, String::from("Failed to open file"));
        let error_array = ErrorArray::new(vec![error_item]);
        let result: UnifiedResult<i32> = UnifiedResult::new(Err(error_array.clone()));

        assert!(!result.is_ok());

        match result {
            UnifiedResult::ResultNoWarns(Err(err_array)) => {
                assert_eq!(err_array.len(), 1);
                let errors = err_array.0.read().unwrap();
                assert_eq!(errors[0].err_type, Errors::OpeningFile);
                assert_eq!(errors[0].err_mesg, "Failed to open file");
            }
            _ => panic!("Expected ResultNoWarns with Err"),
        }
    }

    #[test]
    fn test_error_array_item_from_io_error() {
        let io_error = io::Error::new(io::ErrorKind::Other, "io error");
        let error_item: ErrorArrayItem = io_error.into();
        assert_eq!(error_item.err_type, Errors::InputOutput);
        assert_eq!(error_item.err_mesg, "io error");
    }

    #[test]
    fn test_error_array_item_from_net_error() {
        let addr_parse_error: AddrParseError =
            "invalid address".parse::<net::IpAddr>().unwrap_err();
        let error_item: ErrorArrayItem = addr_parse_error.into();
        assert_eq!(error_item.err_type, Errors::InputOutput);
        assert_eq!(error_item.err_mesg, "invalid IP address syntax");
    }

    #[test]
    fn test_pop_from_empty_array() {
        let mut errors: ErrorArray = ErrorArray::new_container();
        let result: ErrorArrayItem = errors.pop();

        assert_eq!(result.err_type, Errors::GeneralError);
        assert_eq!(result.err_mesg, "No previous error");
    }

    #[test]
    fn test_pop_single_error() {
        let mut errors: ErrorArray = ErrorArray::new_container();

        let error_item: ErrorArrayItem = ErrorArrayItem::new(
            Errors::AuthenticationError,
            String::from("Auth error occurred"),
        );
        errors.push(error_item.clone());

        let result: ErrorArrayItem = errors.pop();

        assert_eq!(result.err_type, error_item.err_type);
        assert_eq!(result.err_mesg, error_item.err_mesg);

        // Ensure the array is empty after popping
        let empty_result: ErrorArrayItem = errors.pop();
        assert_eq!(empty_result.err_type, Errors::GeneralError);
        assert_eq!(empty_result.err_mesg, "No previous error");
    }

    #[test]
    fn test_pop_multiple_errors() {
        let mut errors = ErrorArray::new_container();

        let error_item1 = ErrorArrayItem::new(Errors::AuthenticationError, String::from("First"));
        let error_item2 = ErrorArrayItem::new(Errors::InvalidAuthRequest, String::from("Second"));
        errors.push(error_item1.clone());
        errors.push(error_item2.clone());

        let result2: ErrorArrayItem = errors.pop();
        assert_eq!(result2.err_type, error_item2.err_type);
        assert_eq!(result2.err_mesg, error_item2.err_mesg);

        let result1: ErrorArrayItem = errors.pop();
        assert_eq!(result1.err_type, error_item1.err_type);
        assert_eq!(result1.err_mesg, error_item1.err_mesg);

        // Ensure the array is empty after popping all errors
        let empty_result: ErrorArrayItem = errors.pop();
        assert_eq!(empty_result.err_type, Errors::GeneralError);
        assert_eq!(empty_result.err_mesg, "No previous error");
    }

    #[test]
    fn strip_warning_from_type() {
        let mut warnings = WarningArray::new_container();
        warnings.push(WarningArrayItem::new(Warnings::Warning));

        let okwarning = OkWarning {
            data: String::new(),
            warning: warnings,
        };

        assert_eq!(okwarning.strip(), String::new())
    }
}
