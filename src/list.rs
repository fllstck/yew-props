use yew::prelude::*;

pub struct ListGroup {
    props: ListGroupProps,
}

#[derive(Properties, Clone)]
pub struct ListGroupProps {
    pub children: Children,
}

impl Component for ListGroup {
    type Message = ();
    type Properties = ListGroupProps;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self { props }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        html! {
            <ul class="list-group">{ self.props.children.clone() }</ul>
        }
    }
}

pub struct ListGroupItem {
    props: ListGroupItemProps,
}

#[derive(Properties, Clone)]
pub struct ListGroupItemProps {
    pub children: Children,
    #[prop_or(false)]
    pub active: bool,
}

impl Component for ListGroupItem {
    type Message = ();
    type Properties = ListGroupItemProps;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self { props }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        let mut classes = vec!["list-group-item".to_string()];

        if self.props.active {
            classes.push("active".to_string())
        }

        html! {
            <li class=classes>{ self.props.children.clone() }</li>
        }
    }
}
