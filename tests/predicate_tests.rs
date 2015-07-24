#![feature(plugin)]
#![plugin(speculate)]

extern crate select;
pub use select::dom::Dom;
pub use select::node;
pub use select::predicate::*;

speculate! {
    describe "predicate" {
        before {
            let dom = Dom::from_str("<html><head></head><body>\
<article id='post-0' class='post category-foo tag-bar'>foo</article>\
</body></html>");
            let html = dom.nth(0);
            let head = dom.nth(1);
            let body = dom.nth(2);
            let article = dom.nth(3);
            let foo = dom.nth(4);
        }

        test "()" {
            assert_eq!(().matches(&html), true);
            assert_eq!(().matches(&head), true);
            assert_eq!(().matches(&body), true);
            assert_eq!(().matches(&article), true);
        }

        test "Name()" {
            assert_eq!(Name("html").matches(&html), true);
            assert_eq!(Name("head").matches(&html), false);
            assert_eq!(Name("body").matches(&html), false);
            assert_eq!(Name("html").matches(&head), false);
            assert_eq!(Name("head").matches(&head), true);
            assert_eq!(Name("body").matches(&head), false);
            assert_eq!(Name("html").matches(&body), false);
            assert_eq!(Name("head").matches(&body), false);
            assert_eq!(Name("body").matches(&body), true);
        }

        test "Class()" {
            assert_eq!(Class("post").matches(&html), false);
            assert_eq!(Class("post").matches(&article), true);
            assert_eq!(Class("category-foo").matches(&article), true);
            assert_eq!(Class("tag-bar").matches(&article), true);
            assert_eq!(Class("foo").matches(&article), false);
            assert_eq!(Class("bar").matches(&article), false);
        }

        test "Not()" {
            assert_eq!(Not(Name("html")).matches(&html), false);
            assert_eq!(Not(Name("html")).matches(&head), true);
            assert_eq!(Not(Name("head")).matches(&html), true);
            assert_eq!(Not(Name("head")).matches(&head), false);
        }

        test "Attr()" {
            assert_eq!(Attr("id", "post-0").matches(&html), false);
            assert_eq!(Attr("id", "post-0").matches(&article), true);
        }

        test "Fn(&Node) -> bool" {
            let f = |node: &node::Node| node.name() == Some("html");
            assert_eq!(f.matches(&html), true);
            assert_eq!(f.matches(&head), false);
            assert_eq!(f.matches(&body), false);
        }

        test "Element" {
            assert_eq!(super::Element.matches(&html), true);
            assert_eq!(super::Element.matches(&head), true);
            assert_eq!(super::Element.matches(&body), true);
            assert_eq!(super::Element.matches(&article), true);
            assert_eq!(super::Element.matches(&foo), false);
        }

        test "Text" {
            assert_eq!(super::Text.matches(&html), false);
            assert_eq!(super::Text.matches(&head), false);
            assert_eq!(super::Text.matches(&body), false);
            assert_eq!(super::Text.matches(&article), false);
            assert_eq!(super::Text.matches(&foo), true);
        }

        test "Or()" {
            let html_or_head = Or(Name("html"), Name("head"));
            assert_eq!(html_or_head.matches(&html), true);
            assert_eq!(html_or_head.matches(&head), true);
            assert_eq!(html_or_head.matches(&body), false);
            assert_eq!(html_or_head.matches(&article), false);
            assert_eq!(html_or_head.matches(&foo), false);
        }
    }
}
