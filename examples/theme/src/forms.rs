use ybc::*;
use yew::prelude::*;

use crate::sections::{SectionContent, SectionId};
#[function_component(FormSection)]
pub fn form_section() -> Html {
    // Local state only for file inputs so filenames update when selecting files
    let file1 = use_state(Vec::new);
    let file2 = use_state(Vec::new);
    let file3 = use_state(Vec::new);
    let file4 = use_state(Vec::new);
    let file5 = use_state(Vec::new);
    let file6 = use_state(Vec::new);
    html! {
        <>
            <SectionContent section={SectionId::VerticalForm}>
                <Columns>
                    <Column>
            <ybc::Field label={Some("Name".to_string())}>
                            <ybc::Control>
                <ybc::Input name="name" value="" update={Callback::from(|_: String| {})} placeholder="Text input" />
                            </ybc::Control>
                        </ybc::Field>

                        <ybc::Field label={Some(String::from("Username"))} help={Some(String::from("This username is available"))} help_classes={classes!("is-success")}>
                            <ybc::Control icons_left=true icons_right=true>
                                <ybc::Input name="username" value="bulma" update={Callback::from(|_: String| {})} classes="is-success" placeholder="Text input" />
                                <Icon alignment={Some(ybc::Alignment::Left)} size={Some(ybc::Size::Small)}>
                                    <i class="fa fa-user"></i>
                                </Icon>
                                <Icon alignment={Some(ybc::Alignment::Right)} size={Some(ybc::Size::Small)}>
                                    <i class="fa fa-check"></i>
                                </Icon>
                            </ybc::Control>
                        </ybc::Field>

                        <ybc::Field label={Some(String::from("Email"))} help={Some(String::from("This email is invalid"))} help_classes={classes!("is-danger")}>
                            <ybc::Control icons_left=true icons_right=true>
                                <ybc::Input name="email" value="hello@" update={Callback::from(|_: String| {})} classes="is-danger" placeholder="Email input" />
                                <Icon alignment={Some(ybc::Alignment::Left)} size={Some(ybc::Size::Small)}>
                                    <i class="fa fa-envelope"></i>
                                </Icon>
                                <Icon alignment={Some(ybc::Alignment::Right)} size={Some(ybc::Size::Small)}>
                                    <i class="fa fa-warning"></i>
                                </Icon>
                            </ybc::Control>
                        </ybc::Field>

                        <ybc::Field label={Some(String::from("Subject"))}>
                            <ybc::Control>
                                <ybc::Select name="subject" value="" update={Callback::from(|_: String| {})}>
                                    <option>{"Select dropdown"}</option>
                                    <option>{"With options"}</option>
                                </ybc::Select>
                            </ybc::Control>
                        </ybc::Field>

                        <ybc::Field label={Some(String::from("Files to join"))}>
                            <ybc::Control>
                                <ybc::MultiSelect name="files_to_join" value={vec![]} update={Callback::from(|_: Vec<String>| {})}>
                                    <option>{"Select dropdown"}</option>
                                    <option>{"With options"}</option>
                                </ybc::MultiSelect>
                            </ybc::Control>
                        </ybc::Field>

                        <ybc::Field label={Some(String::from("Message"))}>
                            <ybc::Control>
                                <ybc::TextArea name="message" value="" update={Callback::from(|_: String| {})} placeholder="Textarea" />
                            </ybc::Control>
                        </ybc::Field>

                        <ybc::Field>
                            <ybc::Control>
                                <ybc::Checkbox name="agree" checked={false} update={Callback::from(|_: bool| {})}>
                                    {" I agree to the "}
                                    <a href="#">{"terms and conditions"}</a>
                                </ybc::Checkbox>
                            </ybc::Control>
                        </ybc::Field>

                        <ybc::Field>
                            <ybc::Control>
                                <ybc::Radio name="question" value="Yes" checked_value={None::<String>} update={Callback::from(|_: String| {})}>{" Yes"}</ybc::Radio>
                                {" "}
                                <ybc::Radio name="question" value="No" checked_value={None::<String>} update={Callback::from(|_: String| {})}>{" No"}</ybc::Radio>
                            </ybc::Control>
                        </ybc::Field>

                        <ybc::Field grouped=true>
                            <ybc::Control>
                                <Button classes="is-primary">{"Submit"}</Button>
                            </ybc::Control>
                            <ybc::Control>
                                <Button classes="is-link">{"Cancel"}</Button>
                            </ybc::Control>
                        </ybc::Field>

                        <br />
                        <Subtitle tag="h4">{"Disabled"}</Subtitle>
                        <hr />

                        <ybc::Field>
                            <ybc::Control>
                                <ybc::Input name="disabled_input" value="" update={Callback::from(|_| {})} placeholder="Disabled input" disabled=true />
                            </ybc::Control>
                        </ybc::Field>
                        <ybc::Field>
                            <ybc::Control>
                                <ybc::TextArea name="disabled_textarea" value="" update={Callback::from(|_| {})} placeholder="Disabled textarea" disabled=true />
                            </ybc::Control>
                        </ybc::Field>
                        <ybc::Field>
                            <ybc::Control>
                                <ybc::Checkbox name="remember" checked={false} update={Callback::from(|_| {})} classes="is-disabled" disabled=true>
                                    {" Remember me"}
                                </ybc::Checkbox>
                            </ybc::Control>
                        </ybc::Field>
                        <ybc::Field>
                            <ybc::Control>
                                <ybc::Radio name="question_disabled" value="Yes" checked_value={None::<String>} update={Callback::from(|_| {})} classes="is-disabled" disabled=true>
                                    {" Yes"}
                                </ybc::Radio>
                                {" "}
                                <ybc::Radio name="question_disabled" value="No" checked_value={None::<String>} update={Callback::from(|_| {})} classes="is-disabled" disabled=true>
                                    {" No"}
                                </ybc::Radio>
                            </ybc::Control>
                        </ybc::Field>
                        <ybc::Field grouped=true>
                            <ybc::Control>
                                <Button classes="is-primary" disabled=true>{"Submit"}</Button>
                            </ybc::Control>
                            <ybc::Control>
                                <Button disabled=true>{"Cancel"}</Button>
                            </ybc::Control>
                        </ybc::Field>

                        <br />
                        <Title tag="h3">{"Horizontal Form"}</Title>
                        <hr />

                        <ybc::Field horizontal=true label={Some(String::from("From"))} label_classes={classes!("is-normal", "has-text-left")}>
                            <ybc::Field grouped=true>
                                <ybc::Control classes="is-expanded" icons_left=true>
                                    <ybc::Input name="from_name" value="" update={Callback::from(|_| {})} placeholder="Name" />
                                    <Icon alignment={Some(ybc::Alignment::Left)} size={Some(ybc::Size::Small)}>
                                        <i class="fa fa-user"></i>
                                    </Icon>
                                </ybc::Control>
                            </ybc::Field>
                            <ybc::Field help={Some(String::from("This email is correct"))} help_classes={classes!("is-success")}>
                                <ybc::Control classes="is-expanded" icons_left=true icons_right=true>
                                    <ybc::Input name="from_email" value="alex@smith.com" update={Callback::from(|_: String| {})} classes="is-success" placeholder="Email" />
                                    <Icon alignment={Some(ybc::Alignment::Left)} size={Some(ybc::Size::Small)}>
                                        <i class="fa fa-envelope"></i>
                                    </Icon>
                                    <Icon alignment={Some(ybc::Alignment::Right)} size={Some(ybc::Size::Small)}>
                                        <i class="fa fa-check"></i>
                                    </Icon>
                                </ybc::Control>
                            </ybc::Field>
                        </ybc::Field>

                        <ybc::Field horizontal=true label={Some(String::from("Department"))} label_classes={classes!("is-normal", "has-text-left")}>
                            <ybc::Field classes="is-narrow">
                                <ybc::Control>
                                    <ybc::Select name="department" value="Business development" update={Callback::from(|_: String| {})} classes="is-fullwidth">
                                        <option>{"Business development"}</option>
                                        <option>{"Marketing"}</option>
                                        <option>{"Sales"}</option>
                                    </ybc::Select>
                                </ybc::Control>
                            </ybc::Field>
                        </ybc::Field>

                        <ybc::Field horizontal=true label={Some(String::from("Skills"))} label_classes={classes!("is-normal", "has-text-left")}>
                            <ybc::Control>
                                <ybc::MultiSelect name="skills" value={vec!["Science computer".to_string()]} update={Callback::from(|_: Vec<String>| {})}>
                                    <option>{"Science computer"}</option>
                                    <option>{"Development"}</option>
                                    <option>{"Management"}</option>
                                    <option>{"Relationship"}</option>
                                </ybc::MultiSelect>
                            </ybc::Control>
                        </ybc::Field>

                        <ybc::Field horizontal=true label={Some(String::from("Already a member?"))} label_classes={classes!("is-normal", "has-text-left")}>
                            <ybc::Field classes="is-narrow">
                                <ybc::Control>
                                    <ybc::Radio name="member" value="Yes" checked_value={None::<String>} update={Callback::from(|_| {})}>{" Yes"}</ybc::Radio>
                                    {" "}
                                    <ybc::Radio name="member" value="No" checked_value={None::<String>} update={Callback::from(|_| {})}>{" No"}</ybc::Radio>
                                </ybc::Control>
                            </ybc::Field>
                        </ybc::Field>

                        <ybc::Field horizontal=true label={Some(String::from("Subject"))} label_classes={classes!("is-normal", "has-text-left")}>
                            <ybc::Field help={Some(String::from("This field is required"))} help_classes={classes!("is-danger")}>
                                <ybc::Control>
                                    <ybc::Input name="subject_error" value="" update={Callback::from(|_: String| {})} classes="is-danger" placeholder="e.g. Partnership opportunity" />
                                </ybc::Control>
                            </ybc::Field>
                        </ybc::Field>

                        <ybc::Field horizontal=true label={Some(String::from("Question"))} label_classes={classes!("is-normal", "has-text-left")}>
                            <ybc::Field>
                                <ybc::Control>
                                    <ybc::TextArea name="question" value="" update={Callback::from(|_: String| {})} placeholder="Explain how we can help you" />
                                </ybc::Control>
                            </ybc::Field>
                        </ybc::Field>

                        <ybc::Field horizontal=true label={None::<String>} label_classes={classes!("is-normal", "has-text-left")}>
                            <ybc::Field>
                                <ybc::Control>
                                    <Button classes="is-primary">{"Send message"}</Button>
                                </ybc::Control>
                            </ybc::Field>
                        </ybc::Field>

                        <br />
                        <Title tag="h3">{"File"}</Title>
                        <hr />
                        <ybc::Field>
                            <ybc::File name="resume1" files={(*file1).clone()} update={{ let s=file1.clone(); Callback::from(move |v| s.set(v)) }} selector_icon={html!{<i class="fa fa-upload"></i>}} />
                        </ybc::Field>
                        <ybc::Field>
                            <ybc::File name="resume2" files={(*file2).clone()} update={{ let s=file2.clone(); Callback::from(move |v| s.set(v)) }} has_name={Some("Screen Shot 2017-07-29 at 15.54.25.png".to_string())} selector_icon={html!{<i class="fa fa-upload"></i>}} />
                        </ybc::Field>
                        <ybc::Field>
                            <ybc::File name="resume3" files={(*file3).clone()} update={{ let s=file3.clone(); Callback::from(move |v| s.set(v)) }} classes="is-primary" selector_icon={html!{<i class="fa fa-upload"></i>}} selector_label="Primary file…" />
                        </ybc::Field>
                        <ybc::Field>
                            <ybc::File name="resume4" files={(*file4).clone()} update={{ let s=file4.clone(); Callback::from(move |v| s.set(v)) }} classes="is-info" has_name={Some("Screen Shot 2017-07-29 at 15.54.25.png".to_string())} selector_icon={html!{<i class="fa fa-upload"></i>}} selector_label="Info file…" />
                        </ybc::Field>
                        <ybc::Field>
                            <ybc::File name="resume5" files={(*file5).clone()} update={{ let s=file5.clone(); Callback::from(move |v| s.set(v)) }} classes="is-warning is-boxed" selector_icon={html!{<i class="fa fa-cloud-upload-alt"></i>}} selector_label="Warning file…" />
                        </ybc::Field>
                        <ybc::Field>
                            <ybc::File name="resume6" files={(*file6).clone()} update={{ let s=file6.clone(); Callback::from(move |v| s.set(v)) }} classes="is-danger has-name is-boxed" selector_icon={html!{<i class="fa fa-cloud-upload-alt"></i>}} selector_label="Danger file…" has_name={Some("Screen Shot 2017-07-29 at 15.54.25.png".to_string())} />
                        </ybc::Field>
                    </Column>
                    <Column>
                        <br />
                        <Subtitle tag="h3">{"Styles"}</Subtitle>
                        <hr />
                        <ybc::Field>
                            <ybc::Control>
                                <ybc::Input name="rounded" value="" update={Callback::from(|_| {})} placeholder="Rounded input" rounded=true />
                            </ybc::Control>
                        </ybc::Field>
                        <br />
                        <Subtitle tag="h3">{"Colors"}</Subtitle>
                        <hr />
                        <ybc::Field><ybc::Control><ybc::Input name="c1" value="" update={Callback::from(|_: String| {})} classes="is-primary" placeholder="Primary input" /></ybc::Control></ybc::Field>
                        <ybc::Field><ybc::Control><ybc::Input name="c2" value="" update={Callback::from(|_: String| {})} classes="is-info" placeholder="Info input" /></ybc::Control></ybc::Field>
                        <ybc::Field><ybc::Control><ybc::Input name="c3" value="" update={Callback::from(|_: String| {})} classes="is-success" placeholder="Success input" /></ybc::Control></ybc::Field>
                        <ybc::Field><ybc::Control><ybc::Input name="c4" value="" update={Callback::from(|_: String| {})} classes="is-warning" placeholder="Warning input" /></ybc::Control></ybc::Field>
                        <ybc::Field><ybc::Control><ybc::Input name="c5" value="" update={Callback::from(|_: String| {})} classes="is-danger" placeholder="Danger input" /></ybc::Control></ybc::Field>
                        <br />
                        <Subtitle tag="h3">{"Sizes"}</Subtitle>
                        <hr />
                        <ybc::Field><ybc::Control><ybc::Input name="s1" value="" update={Callback::from(|_| {})} size={Some(ybc::Size::Small)} placeholder="Small input" /></ybc::Control></ybc::Field>
                        <ybc::Field><ybc::Control><ybc::Input name="s2" value="" update={Callback::from(|_| {})} placeholder="Normal input" /></ybc::Control></ybc::Field>
                        <ybc::Field><ybc::Control><ybc::Input name="s3" value="" update={Callback::from(|_| {})} size={Some(ybc::Size::Medium)} placeholder="Medium input" /></ybc::Control></ybc::Field>
                        <ybc::Field><ybc::Control><ybc::Input name="s4" value="" update={Callback::from(|_| {})} size={Some(ybc::Size::Large)} placeholder="Large input" /></ybc::Control></ybc::Field>

                        <ybc::Field><ybc::Control><ybc::Select name="ss1" value="" update={Callback::from(|_| {})} classes="is-small"><option>{"Select dropdown"}</option><option>{"With options"}</option></ybc::Select></ybc::Control></ybc::Field>
                        <ybc::Field><ybc::Control><ybc::Select name="ss2" value="" update={Callback::from(|_| {})}><option>{"Select dropdown"}</option><option>{"With options"}</option></ybc::Select></ybc::Control></ybc::Field>
                        <ybc::Field><ybc::Control><ybc::Select name="ss3" value="" update={Callback::from(|_| {})} size={Some(ybc::Size::Medium)}><option>{"Select dropdown"}</option><option>{"With options"}</option></ybc::Select></ybc::Control></ybc::Field>
                        <ybc::Field><ybc::Control><ybc::Select name="ss4" value="" update={Callback::from(|_| {})} size={Some(ybc::Size::Large)}><option>{"Select dropdown"}</option><option>{"With options"}</option></ybc::Select></ybc::Control></ybc::Field>

                        <ybc::Field label={Some("Small input".to_string())} label_classes={classes!("is-small")}>
                            <ybc::Control icons_left=true icons_right=true>
                                <ybc::Input name="se1" value="" update={Callback::from(|_: String| {})} size={Some(ybc::Size::Small)} r#type={ybc::InputType::Email} placeholder="Email" />
                                <Icon alignment={Some(ybc::Alignment::Left)} size={Some(ybc::Size::Small)}>
                                    <i class="fa fa-envelope"></i>
                                </Icon>
                                <Icon alignment={Some(ybc::Alignment::Right)} size={Some(ybc::Size::Small)}>
                                    <i class="fa fa-check"></i>
                                </Icon>
                            </ybc::Control>
                        </ybc::Field>
                        <ybc::Field label={Some("Normal input".to_string())}>
                            <ybc::Control icons_left=true icons_right=true>
                                <ybc::Input name="ne1" value="" update={Callback::from(|_: String| {})} r#type={ybc::InputType::Email} placeholder="Email" />
                                <Icon alignment={Some(ybc::Alignment::Left)} size={Some(ybc::Size::Small)}>
                                    <i class="fa fa-envelope"></i>
                                </Icon>
                                <Icon alignment={Some(ybc::Alignment::Right)} size={Some(ybc::Size::Small)}>
                                    <i class="fa fa-check"></i>
                                </Icon>
                            </ybc::Control>
                        </ybc::Field>
                        <ybc::Field>
                            <ybc::Control icons_left=true icons_right=true>
                                <ybc::Input name="ne2" value="" update={Callback::from(|_| {})} r#type={ybc::InputType::Email} placeholder="Email" />
                                <Icon alignment={Some(ybc::Alignment::Left)}>
                                    <i class="fa fa-envelope"></i>
                                </Icon>
                                <Icon alignment={Some(ybc::Alignment::Right)}>
                                    <i class="fa fa-check"></i>
                                </Icon>
                            </ybc::Control>
                        </ybc::Field>
                        <ybc::Field label={Some("Medium input".to_string())} label_classes={classes!("is-medium")}>
                            <ybc::Control icons_left=true icons_right=true>
                                <ybc::Input name="me1" value="" update={Callback::from(|_: String| {})} size={Some(ybc::Size::Medium)} r#type={ybc::InputType::Email} placeholder="Email" />
                                <Icon alignment={Some(ybc::Alignment::Left)} size={Some(ybc::Size::Small)}>
                                    <i class="fa fa-envelope"></i>
                                </Icon>
                                <Icon alignment={Some(ybc::Alignment::Right)} size={Some(ybc::Size::Small)}>
                                    <i class="fa fa-check"></i>
                                </Icon>
                            </ybc::Control>
                        </ybc::Field>
                        <ybc::Field>
                            <ybc::Control icons_left=true icons_right=true>
                                <ybc::Input name="me2" value="" update={Callback::from(|_| {})} size={Some(ybc::Size::Medium)} r#type={ybc::InputType::Email} placeholder="Email" />
                                <Icon alignment={Some(ybc::Alignment::Left)}>
                                    <i class="fa fa-envelope"></i>
                                </Icon>
                                <Icon alignment={Some(ybc::Alignment::Right)}>
                                    <i class="fa fa-check"></i>
                                </Icon>
                            </ybc::Control>
                        </ybc::Field>
                        <ybc::Field>
                            <ybc::Control icons_left=true icons_right=true>
                                <ybc::Input name="me3" value="" update={Callback::from(|_| {})} size={Some(ybc::Size::Medium)} r#type={ybc::InputType::Email} placeholder="Email" />
                                <Icon alignment={Some(ybc::Alignment::Left)} size={Some(ybc::Size::Medium)}>
                                    <i class="fa fa-envelope"></i>
                                </Icon>
                                <Icon alignment={Some(ybc::Alignment::Right)} size={Some(ybc::Size::Medium)}>
                                    <i class="fa fa-check"></i>
                                </Icon>
                            </ybc::Control>
                        </ybc::Field>
                        <ybc::Field label={Some("Large input".to_string())} label_classes={classes!("is-large")}>
                            <ybc::Control icons_left=true icons_right=true>
                                <ybc::Input name="le1" value="" update={Callback::from(|_: String| {})} size={Some(ybc::Size::Large)} r#type={ybc::InputType::Email} placeholder="Email" />
                                <Icon alignment={Some(ybc::Alignment::Left)} size={Some(ybc::Size::Small)}>
                                    <i class="fa fa-envelope"></i>
                                </Icon>
                                <Icon alignment={Some(ybc::Alignment::Right)} size={Some(ybc::Size::Small)}>
                                    <i class="fa fa-check"></i>
                                </Icon>
                            </ybc::Control>
                        </ybc::Field>
                        <ybc::Field>
                            <ybc::Control icons_left=true icons_right=true>
                                <ybc::Input name="le2" value="" update={Callback::from(|_| {})} size={Some(ybc::Size::Large)} r#type={ybc::InputType::Email} placeholder="Email" />
                                <Icon alignment={Some(ybc::Alignment::Left)}>
                                    <i class="fa fa-envelope"></i>
                                </Icon>
                                <Icon alignment={Some(ybc::Alignment::Right)}>
                                    <i class="fa fa-check"></i>
                                </Icon>
                            </ybc::Control>
                        </ybc::Field>
                        <ybc::Field>
                            <ybc::Control icons_left=true icons_right=true>
                                <ybc::Input name="le3" value="" update={Callback::from(|_| {})} size={Some(ybc::Size::Large)} r#type={ybc::InputType::Email} placeholder="Email" />
                                <Icon alignment={Some(ybc::Alignment::Left)} size={Some(ybc::Size::Medium)}>
                                    <i class="fa fa-envelope"></i>
                                </Icon>
                                <Icon alignment={Some(ybc::Alignment::Right)} size={Some(ybc::Size::Medium)}>
                                    <i class="fa fa-check"></i>
                                </Icon>
                            </ybc::Control>
                        </ybc::Field>
                        <ybc::Field>
                            <ybc::Control icons_left=true icons_right=true>
                                <ybc::Input name="le4" value="" update={Callback::from(|_| {})} size={Some(ybc::Size::Large)} r#type={ybc::InputType::Email} placeholder="Email" />
                                <Icon alignment={Some(ybc::Alignment::Left)} size={Some(ybc::Size::Large)}>
                                    <i class="fa fa-envelope"></i>
                                </Icon>
                                <Icon alignment={Some(ybc::Alignment::Right)} size={Some(ybc::Size::Large)}>
                                    <i class="fa fa-check"></i>
                                </Icon>
                            </ybc::Control>
                        </ybc::Field>

                        <br />
                        <Subtitle tag="h4">{"With Font Awesome icons"}</Subtitle>
                        <hr />
                        <ybc::Field>
                            <ybc::Control icons_left=true>
                                <ybc::Input name="fa_email" value="" update={Callback::from(|_| {})} r#type={ybc::InputType::Email} placeholder="Email" />
                                <Icon alignment={Some(ybc::Alignment::Left)} size={Some(ybc::Size::Small)}>
                                    <i class="fa fa-envelope"></i>
                                </Icon>
                            </ybc::Control>
                        </ybc::Field>
                        <ybc::Field>
                            <ybc::Control icons_left=true>
                                <ybc::Input name="fa_pass" value="" update={Callback::from(|_| {})} r#type={ybc::InputType::Password} placeholder="Password" />
                                <Icon alignment={Some(ybc::Alignment::Left)} size={Some(ybc::Size::Small)}>
                                    <i class="fa fa-lock"></i>
                                </Icon>
                            </ybc::Control>
                        </ybc::Field>
                        <ybc::Field>
                            <ybc::Control>
                                <Button classes="is-success">{"Login"}</Button>
                            </ybc::Control>
                        </ybc::Field>

                        <br />
                        <Title tag="h3">{"Form addons"}</Title>
                        <hr />
                        <ybc::Field addons=true>
                            <ybc::Control>
                                <ybc::Input name="addon1" value="" update={Callback::from(|_| {})} placeholder="Find a repository" />
                            </ybc::Control>
                            <ybc::Control>
                                <Button classes="is-info">{"Search"}</Button>
                            </ybc::Control>
                        </ybc::Field>
                        <ybc::Field addons=true>
                            <ybc::Control>
                                <ybc::Input name="addon2" value="" update={Callback::from(|_| {})} size={Some(ybc::Size::Large)} placeholder="Find a repository" />
                            </ybc::Control>
                            <ybc::Control>
                                <Button classes="is-info is-large">{"Search"}</Button>
                            </ybc::Control>
                        </ybc::Field>
                        <ybc::Field addons=true>
                            <ybc::Control>
                                <ybc::Select name="currency1" value="" update={Callback::from(|_: String| {})}>
                                    <option>{"$"}</option>
                                    <option>{"£"}</option>
                                    <option>{"€"}</option>
                                </ybc::Select>
                            </ybc::Control>
                            <ybc::Control>
                                <ybc::Input name="amount1" value="" update={Callback::from(|_: String| {})} placeholder="Amount of money" />
                            </ybc::Control>
                            <ybc::Control>
                                <Button>{"Transfer"}</Button>
                            </ybc::Control>
                        </ybc::Field>
                        <ybc::Field addons=true>
                            <ybc::Control>
                                <ybc::Select name="currency2" value="" update={Callback::from(|_: String| {})}>
                                    <option>{"$"}</option>
                                    <option>{"£"}</option>
                                    <option>{"€"}</option>
                                </ybc::Select>
                            </ybc::Control>
                            <ybc::Control classes="is-expanded">
                                <ybc::Input name="amount2" value="" update={Callback::from(|_: String| {})} placeholder="Amount of money" />
                            </ybc::Control>
                            <ybc::Control>
                                <Button>{"Transfer"}</Button>
                            </ybc::Control>
                        </ybc::Field>
                        <ybc::Field addons=true>
                            <ybc::Control classes="is-expanded">
                                <ybc::Select name="country" value="Argentina" update={Callback::from(|_: String| {})} classes="is-fullwidth">
                                    <option value="Argentina">{"Argentina"}</option>
                                    <option value="Bolivia">{"Bolivia"}</option>
                                    <option value="Brazil">{"Brazil"}</option>
                                    <option value="Chile">{"Chile"}</option>
                                    <option value="Colombia">{"Colombia"}</option>
                                    <option value="Ecuador">{"Ecuador"}</option>
                                    <option value="Guyana">{"Guyana"}</option>
                                    <option value="Paraguay">{"Paraguay"}</option>
                                    <option value="Peru">{"Peru"}</option>
                                    <option value="Suriname">{"Suriname"}</option>
                                    <option value="Uruguay">{"Uruguay"}</option>
                                    <option value="Venezuela">{"Venezuela"}</option>
                                </ybc::Select>
                            </ybc::Control>
                            <ybc::Control>
                                <Button classes="is-primary">{"Choose"}</Button>
                            </ybc::Control>
                        </ybc::Field>
                        <ybc::Field grouped=true>
                            <ybc::Control classes="is-expanded">
                                <ybc::Input name="repo" value="" update={Callback::from(|_: String| {})} placeholder="Find a repository" />
                            </ybc::Control>
                            <ybc::Control>
                                <Button classes="is-info">{"Search"}</Button>
                            </ybc::Control>
                        </ybc::Field>
                    </Column>
                </Columns>
            </SectionContent>
        </>
    }
}
