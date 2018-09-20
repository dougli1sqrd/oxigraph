///! Provides ready to use NamedNode for basic RDF vocabularies

pub mod rdf {
    use model::named_node::NamedNode;
    use std::str::FromStr;

    lazy_static! {
        pub static ref FIRST: NamedNode =
            NamedNode::from_str("http://www.w3.org/1999/02/22-rdf-syntax-ns#first").unwrap();
        pub static ref HTML: NamedNode =
            NamedNode::from_str("http://www.w3.org/1999/02/22-rdf-syntax-ns#HTML").unwrap();
        pub static ref LANG_STRING: NamedNode =
            NamedNode::from_str("http://www.w3.org/1999/02/22-rdf-syntax-ns#langString").unwrap();
        pub static ref NIL: NamedNode =
            NamedNode::from_str("http://www.w3.org/1999/02/22-rdf-syntax-ns#nil").unwrap();
        pub static ref OBJECT: NamedNode =
            NamedNode::from_str("http://www.w3.org/1999/02/22-rdf-syntax-ns#object").unwrap();
        pub static ref PREDICATE: NamedNode =
            NamedNode::from_str("http://www.w3.org/1999/02/22-rdf-syntax-ns#predicate").unwrap();
        pub static ref REST: NamedNode =
            NamedNode::from_str("http://www.w3.org/1999/02/22-rdf-syntax-ns#rest").unwrap();
        pub static ref STATEMENT: NamedNode =
            NamedNode::from_str("http://www.w3.org/1999/02/22-rdf-syntax-ns#Statement").unwrap();
        pub static ref SUBJECT: NamedNode =
            NamedNode::from_str("http://www.w3.org/1999/02/22-rdf-syntax-ns#subject").unwrap();
        pub static ref TYPE: NamedNode =
            NamedNode::from_str("http://www.w3.org/1999/02/22-rdf-syntax-ns#type").unwrap();
        pub static ref XML_LITERAL: NamedNode =
            NamedNode::from_str("http://www.w3.org/1999/02/22-rdf-syntax-ns#XMLLiteral").unwrap();
    }
}

pub mod rdfs {
    use model::named_node::NamedNode;
    use std::str::FromStr;

    lazy_static! {
        pub static ref COMMENT: NamedNode =
            NamedNode::from_str("http://www.w3.org/2000/01/rdf-schema#comment").unwrap();
    }
}

pub mod xsd {
    ///! NamedNodes for [RDF compatible XSD datatypes](https://www.w3.org/TR/rdf11-concepts/#dfn-rdf-compatible-xsd-types)
    use model::named_node::NamedNode;
    use std::str::FromStr;

    lazy_static! {
        /// true, false
        pub static ref BOOLEAN: NamedNode =
            NamedNode::from_str("http://www.w3.org/2001/XMLSchema#boolean").unwrap();
        /// Dates (yyyy-mm-dd) with or without timezone
        pub static ref DATE: NamedNode =
            NamedNode::from_str("http://www.w3.org/2001/XMLSchema#date").unwrap();
        /// Duration of time (days, hours, minutes, seconds only)
        pub static ref DAY_TIME_DURATION: NamedNode =
            NamedNode::from_str("http://www.w3.org/2001/XMLSchema#dayTimeDuration").unwrap();
        /// Date and time with or without timezone
        pub static ref DATE_TIME: NamedNode =
            NamedNode::from_str("http://www.w3.org/2001/XMLSchema#dateTime").unwrap();
        /// Date and time with required timezone
        pub static ref DATE_TIME_STAMP: NamedNode =
            NamedNode::from_str("http://www.w3.org/2001/XMLSchema#dateTimeStamp").unwrap();
        /// Arbitrary-precision decimal numbers
        pub static ref DECIMAL: NamedNode =
            NamedNode::from_str("http://www.w3.org/2001/XMLSchema#decimal").unwrap();
        /// 64-bit floating point numbers incl. ±Inf, ±0, NaN
        pub static ref DOUBLE: NamedNode =
            NamedNode::from_str("http://www.w3.org/2001/XMLSchema#double").unwrap();
        /// Duration of time
        pub static ref DURATION: NamedNode =
            NamedNode::from_str("http://www.w3.org/2001/XMLSchema#duration").unwrap();
        /// 32-bit floating point numbers incl. ±Inf, ±0, NaN
        pub static ref FLOAT: NamedNode =
            NamedNode::from_str("http://www.w3.org/2001/XMLSchema#float").unwrap();
        /// Gregorian calendar day of the month
        pub static ref G_DAY: NamedNode =
            NamedNode::from_str("http://www.w3.org/2001/XMLSchema#gDay").unwrap();
        /// Gregorian calendar month
        pub static ref G_MONTH: NamedNode =
            NamedNode::from_str("http://www.w3.org/2001/XMLSchema#gMonth").unwrap();
        /// Gregorian calendar month and day
        pub static ref G_MONTH_DAY: NamedNode =
            NamedNode::from_str("http://www.w3.org/2001/XMLSchema#gMonthDay").unwrap();
        /// Gregorian calendar year
        pub static ref G_YEAR: NamedNode =
            NamedNode::from_str("http://www.w3.org/2001/XMLSchema#gYear").unwrap();
        /// Gregorian calendar year and month
        pub static ref G_YEAR_MONTH: NamedNode =
            NamedNode::from_str("http://www.w3.org/2001/XMLSchema#gYearMonth").unwrap();
        /// Arbitrary-size integer numbers
        pub static ref INTEGER: NamedNode =
            NamedNode::from_str("http://www.w3.org/2001/XMLSchema#integer").unwrap();
        /// Times (hh:mm:ss.sss…) with or without timezone
        pub static ref TIME: NamedNode =
            NamedNode::from_str("http://www.w3.org/2001/XMLSchema#time").unwrap();
        /// Character strings (but not all Unicode character strings)
        pub static ref STRING: NamedNode =
            NamedNode::from_str("http://www.w3.org/2001/XMLSchema#string").unwrap();
        /// Duration of time (months and years only)
        pub static ref YEAR_MONTH_DURATION: NamedNode =
            NamedNode::from_str("http://www.w3.org/2001/XMLSchema#yearMonthDuration").unwrap();
    }
}
