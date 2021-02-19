use crate::components::profile::Profile;
use crate::components::user::User;
use yew::prelude::*;
use yew_functional::function_component;

#[function_component(Card)]
pub fn card() -> Html {
    html! {
        <div class="head-block-user">
        <div class="block head-block body-border">
          <div class="card">
            // <UserMini isShow={this.state.isShow} name={this.props.user.name} avatar={this.props.user.avatar} />
            <User />
            <Profile />
          </div>
        </div>
      </div>
    }
}
