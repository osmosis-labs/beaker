use pulldown_cmark::{Event, HeadingLevel, LinkType, Tag};

#[derive(Clone)]
pub struct Document<'a>(pub Vec<Event<'a>>);

impl<'a> Document<'a> {
    pub fn new(events: Vec<Event<'a>>) -> Self {
        Document(events)
    }
    pub fn header(&mut self, text: String, level: HeadingLevel) {
        self.0.push(Event::Start(Tag::Heading(level, None, vec![])));
        self.0.push(Event::Text(text.into()));
        self.0.push(Event::End(Tag::Heading(level, None, vec![])));
    }

    pub fn header_code(&mut self, text: String, level: HeadingLevel) {
        self.0.push(Event::Start(Tag::Heading(level, None, vec![])));
        self.0.push(Event::Code(text.into()));
        self.0.push(Event::End(Tag::Heading(level, None, vec![])));
    }

    pub fn paragraph(&mut self, text: String) {
        self.0.push(Event::Start(Tag::Paragraph));
        self.0.push(Event::Text(text.into()));
        self.0.push(Event::End(Tag::Paragraph));
    }

    pub fn link(&mut self, text: String, link: String) {
        self.0.push(Event::Start(Tag::Link(
            LinkType::Inline,
            link.clone().into(),
            "".into(),
        )));

        self.0.push(Event::Text(text.into()));

        self.0.push(Event::End(Tag::Link(
            LinkType::Inline,
            link.into(),
            "".into(),
        )));
    }
}
