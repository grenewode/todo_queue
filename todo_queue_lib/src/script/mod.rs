#[cfg_attr(rustfmt, rustfmt_skip)] mod parser;

#[cfg(test)]
mod tests {
    use super::parser::*;
    use list::Status;
    use range::{Limit, Range};
    use query::{Filter, Query};

    #[test]
    fn parser_status() {
        assert!(parse_status("").is_err());
        assert!(parse_status("HADASD AS").is_err());
        assert_eq!(parse_status("%Completed"), Ok(Status::Completed));
        assert_eq!(parse_status("%Waiting"), Ok(Status::Waiting));
        assert_eq!(parse_status("%Queuing"), Ok(Status::Queuing));
        assert_eq!(parse_status("%Working"), Ok(Status::Working));
    }

    #[test]
    fn parser_range_status() {
        assert!(parse_range_status("").is_err());
        assert!(parse_range_status("HADASD AS").is_err());
        assert_eq!(
            parse_range_status("( %Waiting ... %Completed ]"),
            Ok(Range::new(
                Limit::Excludes(Status::Waiting),
                Limit::Includes(Status::Completed)
            ))
        );
        assert_eq!(
            parse_range_status("( %Waiting...inf ]"),
            Ok(Range::new(Limit::Excludes(Status::Waiting), Limit::Inf))
        );
        assert_eq!(
            parse_range_status("( %Waiting... ]"),
            Ok(Range::new(Limit::Excludes(Status::Waiting), Limit::Inf))
        );
        assert_eq!(
            parse_range_status("%Waiting"),
            Ok(Range::eq(Status::Waiting))
        );
    }

    #[test]
    fn parser_filter() {
        assert!(parse_filter("").is_err());
        assert_eq!(parse_filter("hello"), Ok(Filter::name("hello")));
        assert_eq!(parse_filter("#tag"), Ok(Filter::tag("tag")));
        assert_eq!(
            parse_filter("hello goodbye"),
            Ok(Filter::name("hello") | Filter::name("goodbye"))
        );
        assert_eq!(
            parse_filter("hello & goodbye"),
            Ok(Filter::name("hello") & Filter::name("goodbye"))
        );
        assert_eq!(
            parse_filter("hello & %work"),
            Ok(Filter::name("hello") & Filter::status(Status::Working))
        );
        assert_eq!(
            parse_filter("(%work) hello"),
            Ok(Filter::status(Status::Working) | Filter::name("hello"))
        );
        assert_eq!(
            parse_filter("(%work) !hello"),
            Ok(Filter::status(Status::Working) | !Filter::name("hello"))
        );
        assert_eq!(
            parse_filter("!%work !hello"),
            Ok(!Filter::status(Status::Working) | !Filter::name("hello"))
        );
    }

    #[test]
    fn parser_query() {
        assert!(parse_query("").is_err());
        assert_eq!(parse_query("hello"), Ok(Filter::name("hello").into()));
        assert_eq!(parse_query("#tag"), Ok(Filter::tag("tag").into()));
        assert_eq!(
            parse_query("!%work !hello => %queue"),
            Ok(
                Query::from(!Filter::status(Status::Working) | !Filter::name("hello"))
                    .then(Filter::status(Status::Queuing))
            )
        );
        assert_eq!(
            parse_query("!%working !hello => %queued => #tag"),
            Ok(
                Query::from(!Filter::status(Status::Working) | !Filter::name("hello"))
                    .then(Filter::status(Status::Queuing))
                    .then(Filter::tag("tag"))
            )
        );
    }
}
