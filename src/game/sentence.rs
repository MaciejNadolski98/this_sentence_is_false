use bevy::{ecs::relationship::RelatedSpawnerCommands, prelude::*, text::LineHeight};

use crate::{game::{checkbox, levels::{CurrentLevel, Level, SentenceDescription}, Checkbox, CheckboxContainer, TextBox}, states::GameState};

pub struct SentencePlugin;

impl Plugin for SentencePlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<Selected>()
            .add_systems(Update, (button_interaction, update_text, background_color_transition))
            .add_systems(Update, level_transition.run_if(resource_changed::<CurrentLevel>.and(in_state(GameState::InGame))));
    }
}

#[derive(Component, Clone, Copy, PartialEq, Debug)]
pub enum Value {
    Id(u32),
    Bool(bool),
    Number(u32),
}

impl Value {
    fn to_string(&self, sentence_id: SentenceId) -> String {
        match *self {
            Value::Id(id) => {
                if id == sentence_id.0 {
                    "This".to_string()
                } else if id % 10 == 1 {
                    format!("{id}st")
                } else if id % 10 == 2 {
                    format!("{id}nd")
                } else if id % 10 == 3 {
                    format!("{id}rd")
                } else {
                    format!("{id}th")
                }
            }
            Value::Bool(b) => format!("{b}"),
            Value::Number(n) => format!(" {n} "),
        }
    }
}

impl Into<BackgroundColor> for Value {
    fn into(self) -> BackgroundColor {
        match self {
            Value::Id(_) => BackgroundColor(Color::srgb_u8(225, 111, 124)),
            Value::Bool(_) => BackgroundColor(Color::srgb_u8(225, 134, 0)),
            Value::Number(_) => BackgroundColor(Color::srgb_u8(242, 220, 93)),
        }
    }
}

#[derive(Component, Clone, Copy)]
pub struct SentenceId(pub u32);

#[derive(Component, Debug, Clone, Copy, PartialEq)]
#[require(Node)]
pub enum Sentence {
    IdSentenceIsBool,
    ThereAreNumberOfBoolSentences,
    ClosestBoolIsNumberAway,
    ThereAreNumberOfAlternatingGroups,
    IdSentenceAndIdSentenceAreTheSame,
    IdSentenceAndIdSentenceAreDifferent,
}

pub fn level_transition(
    level: Res<CurrentLevel>,
    mut commands: Commands,
) {
    commands.run_system_cached(despawn_level);
    commands.run_system_cached_with(spawn_level, Level::get(level.0).clone());
}

pub fn despawn_level(
    mut commands: Commands,
    sentences: Query<Entity, With<Sentence>>,
    checkboxes: Query<Entity, With<Checkbox>>,
) {
    for entity in sentences.iter() {
        commands.entity(entity).despawn();
    }
    for entity in checkboxes.iter() {
        commands.entity(entity).despawn();
    }
}

pub fn spawn_level(
    level: In<Level>,
    mut commands: Commands,
    checkbox_container: Single<Entity, With<CheckboxContainer>>,
    text_box: Single<Entity, With<TextBox>>,
    asset_server: Res<AssetServer>,
) {
    commands.entity(*text_box).with_children(|commands| {
        for (i, sentence) in level.sentences.iter().enumerate() {
            let sentence_id = SentenceId(i as u32 + 1);
            spawn_sentence(
                sentence_id.0, 
                commands,
                sentence.clone(),
            );
        }
    });

    commands.entity(*checkbox_container).with_children(|commands| {
        for _ in 0..level.sentences.len() {
            commands.spawn(checkbox(asset_server.clone()));
        }
    });
}

pub fn spawn_sentence(
    sentence_id: u32,
    commands: &mut RelatedSpawnerCommands<ChildOf>,
    sentence_description: SentenceDescription,
) {
    let SentenceDescription { sentence, values } = sentence_description;
    match sentence {
        Sentence::IdSentenceIsBool =>
            commands.spawn((
                Sentence::IdSentenceIsBool,
                SentenceId(sentence_id),
                related!(Children[
                    plain_text(format!("{}. ", sentence_id)),
                    text(values[0]),
                    plain_text(" sentence is "),
                    text(values[1]),
                ]),
            )),
        Sentence::ThereAreNumberOfBoolSentences => 
            commands.spawn((
                Sentence::ThereAreNumberOfBoolSentences,
                SentenceId(sentence_id),
                related!(Children[
                    plain_text(format!("{}. ", sentence_id)),
                    plain_text("There are "),
                    text(values[0]),
                    plain_text(" sentences that are "),
                    text(values[1]),
                ]),
            )),
        Sentence::ClosestBoolIsNumberAway =>
            commands.spawn((
                Sentence::ClosestBoolIsNumberAway,
                SentenceId(sentence_id),
                related!(Children[
                    plain_text(format!("{}. ", sentence_id)),
                    plain_text("The closest "),
                    text(values[0]),
                    plain_text(" sentence is "),
                    text(values[1]),
                    plain_text(" spots away"),
                ]),
            )),
        Sentence::ThereAreNumberOfAlternatingGroups =>
            commands.spawn((
                Sentence::ThereAreNumberOfAlternatingGroups,
                SentenceId(sentence_id),
                related!(Children[
                    plain_text(format!("{}. ", sentence_id)),
                    plain_text("There are "),
                    text(values[0]),
                    plain_text(" alternating groups"),
                ]),
            )),
        Sentence::IdSentenceAndIdSentenceAreTheSame =>
            commands.spawn((
                Sentence::IdSentenceAndIdSentenceAreTheSame,
                SentenceId(sentence_id),
                related!(Children[
                    plain_text(format!("{}. ", sentence_id)),
                    plain_text("Both "),
                    text(values[0]),
                    plain_text(" and "),
                    text(values[1]),
                    plain_text(" sentences have the same truth value"),
                ]),
            )),
        Sentence::IdSentenceAndIdSentenceAreDifferent =>
            commands.spawn((
                Sentence::IdSentenceAndIdSentenceAreDifferent,
                SentenceId(sentence_id),
                related!(Children[
                    plain_text(format!("{}. ", sentence_id)),
                    plain_text("Both "),
                    text(values[0]),
                    plain_text(" and "),
                    text(values[1]),
                    plain_text(" sentences have the opposite truth values"),
                ]),
            )),
    };
}

fn plain_text<S: Into<String> + Clone>(text: S) -> impl Bundle {
    (
        Name::new(text.clone().into()),
        Text::new(text),
        TextColor(Color::BLACK),
        TextFont {
            font_size: 20.0,
            line_height: LineHeight::Px(48.0),
            ..default()
        },
    )
}

fn text(value: Value) -> impl Bundle {
    let background_color: BackgroundColor = value.into();
    (
        Name::new("Text"),
        Text::new(""),
        Button,
        value,
        TextColor(Color::BLACK),
        TextFont {
            font_size: 20.0,
            line_height: LineHeight::Px(48.0),
            ..default()
        },
        BorderRadius::all(Val::Px(20.0)),
        Outline { color: Color::BLACK, ..default()},
        background_color,
    )
}

fn update_text(
    value: Query<(Entity, &Value, &mut Text), Changed<Value>>,
    child_of: Query<&ChildOf>,
    sentence_ids: Query<&SentenceId>,
) {
    for (entity, &value, mut text) in value {
        let &ChildOf(parent) = child_of.get(entity).unwrap();
        let &sentence_id = sentence_ids.get(parent).unwrap();
        *text = Text::new(value.to_string(sentence_id));
    }
}

#[derive(Resource, Default, PartialEq, Debug)]
struct Selected(Option<Entity>);

fn button_interaction(
    mut commands: Commands,
    interaction_query: Query<(Entity, &Interaction, &mut BackgroundColor), (Changed<Interaction>, With<Value>)>,
    mut selected: ResMut<Selected>,
    mut outlines: Query<&mut Outline>,
) {
    for (entity, interaction, mut color) in interaction_query {
        match *interaction {
            Interaction::Pressed => {
                if let Some(selected_entity) = selected.0 {
                    outlines.get_mut(selected_entity).unwrap().width = Val::Px(0.0);
                    commands.run_system_cached_with(swap, (entity, selected_entity));
                    selected.0 = None;
                } else {
                    selected.0 = Some(entity);
                    outlines.get_mut(entity).unwrap().width = Val::Px(1.0);
                }
            }
            Interaction::Hovered => {
                color.0.set_alpha(0.5);
            }
            Interaction::None => {
                color.0.set_alpha(1.0);
            }
        }
    }
}

fn swap(
    pair: In<(Entity, Entity)>,
    mut value: Query<&mut Value>,
) {
    let (entity1, entity2) = *pair;
    if entity1 == entity2 {
        return;
    }

    let [mut value1, mut value2] = value.get_many_mut([entity1, entity2]).unwrap();

    match (*value1, *value2) {
        (Value::Id(id1), Value::Id(id2)) => {
            *value1 = Value::Id(id2);
            *value2 = Value::Id(id1);
        }
        (Value::Bool(b1), Value::Bool(b2)) => {
            *value1 = Value::Bool(b2);
            *value2 = Value::Bool(b1);
        }
        (Value::Number(n1), Value::Number(n2)) => {
            *value1 = Value::Number(n2);
            *value2 = Value::Number(n1);
        }
        _ => {
            return;
        }
    }
}

#[derive(Component)]
struct BackgroundColorTransition {
    start_color: Color,
    end_color: Color,
    timer: Timer,
}

impl BackgroundColorTransition {
    pub fn new(start_color: Color, end_color: Color, duration: f32) -> Self {
        Self {
            start_color,
            end_color,
            timer: Timer::from_seconds(duration, TimerMode::Once),
        }
    }
}

pub fn evaluate_sentences(
    mut commands: Commands,
    sentences: Query<(Entity, &Sentence, &SentenceId)>,
    children: Query<&Children>,
    values: Query<&Value>,
    checkbox_container: Single<&Children, With<CheckboxContainer>>,
    check_boxes: Query<&Checkbox>,
    mut current_level: ResMut<CurrentLevel>,
) {
    let mut all_consistent = true;
    for (sentence_entity, &sentence, &SentenceId(sentence_id)) in sentences {
        let texts = children.get(sentence_entity).unwrap().iter().collect::<Vec<_>>();
        let checkboxes = (*checkbox_container).iter().collect::<Vec<_>>();
        let truth = match sentence {
            Sentence::IdSentenceIsBool => {
                let &Value::Id(id) = values.get(texts[1]).unwrap() else { panic!("Expected Value::Id") };
                let &Value::Bool(truth) = values.get(texts[3]).unwrap() else { panic!("Expected Value::Bool") };

                let &Checkbox(target_truth) = check_boxes.get(checkboxes[(id - 1) as usize]).unwrap();

                target_truth == truth
            }
            Sentence::ThereAreNumberOfBoolSentences => {
                let &Value::Number(number) = values.get(texts[2]).unwrap() else { panic!("Expected Value::Number") };
                let &Value::Bool(truth) = values.get(texts[4]).unwrap() else { panic!("Expected Value::Bool") };

                let mut count = 0;
                for &checkbox in checkboxes.iter() {
                    let &Checkbox(target_truth) = check_boxes.get(checkbox).unwrap();
                    if target_truth == truth {
                        count += 1;
                    }
                }
                count == number
            }
            Sentence::ClosestBoolIsNumberAway => {
                let &Value::Bool(truth) = values.get(texts[2]).unwrap() else { panic!("Expected Value::Bool") };
                let &Value::Number(n) = values.get(texts[4]).unwrap() else { panic!("Expected Value::Id") };

                let all_within_range_are_not_truth = {
                    let mut ret = true;
                    let (start, stop) = (sentence_id.saturating_sub(n) + 1, sentence_id + n - 1);
                    for i in start..=stop {
                        if i > checkboxes.len() as u32 || i == sentence_id {
                            continue;
                        }

                        let &Checkbox(target_truth) = check_boxes.get(checkboxes[(i - 1) as usize]).unwrap();
                        if target_truth == truth {
                            ret = false;
                            break;
                        }
                    }

                    ret
                };

                let one_at_distance_is_truth = {
                    let mut ret = false;
                    if sentence_id > n {
                        let &Checkbox(target_truth) = check_boxes.get(checkboxes[(sentence_id - n - 1) as usize]).unwrap();
                        if target_truth == truth {
                            ret = true;
                        }
                    }
                    if sentence_id + n < checkboxes.len() as u32 + 1 {
                        let &Checkbox(target_truth) = check_boxes.get(checkboxes[(sentence_id + n - 1) as usize]).unwrap();
                        if target_truth == truth {
                            ret = true;
                        }
                    }

                    ret
                };

                all_within_range_are_not_truth && one_at_distance_is_truth
            }
            Sentence::ThereAreNumberOfAlternatingGroups => {
                let &Value::Number(number) = values.get(texts[2]).unwrap() else { panic!("Expected Value::Number") };

                let mut count = 0;
                let mut last_truth = None;

                for &checkbox in checkboxes.iter() {
                    let &Checkbox(target_truth) = check_boxes.get(checkbox).unwrap();
                    if Some(target_truth) != last_truth {
                        count += 1;
                        last_truth = Some(target_truth);
                    }
                }

                count == number
            }
            Sentence::IdSentenceAndIdSentenceAreTheSame => {
                let &Value::Id(id1) = values.get(texts[2]).unwrap() else { panic!("Expected Value::Id") };
                let &Value::Id(id2) = values.get(texts[4]).unwrap() else { panic!("Expected Value::Id") };

                let &Checkbox(target_truth1) = check_boxes.get(checkboxes[(id1 - 1) as usize]).unwrap();
                let &Checkbox(target_truth2) = check_boxes.get(checkboxes[(id2 - 1) as usize]).unwrap();

                target_truth1 == target_truth2
            }
            Sentence::IdSentenceAndIdSentenceAreDifferent => {
                let &Value::Id(id1) = values.get(texts[2]).unwrap() else { panic!("Expected Value::Id") };
                let &Value::Id(id2) = values.get(texts[4]).unwrap() else { panic!("Expected Value::Id") };

                let &Checkbox(target_truth1) = check_boxes.get(checkboxes[(id1 - 1) as usize]).unwrap();
                let &Checkbox(target_truth2) = check_boxes.get(checkboxes[(id2 - 1) as usize]).unwrap();

                target_truth1 != target_truth2
            }
        };

        let &Checkbox(target_truth) = check_boxes.get(checkboxes[(sentence_id - 1) as usize]).unwrap();
        let consistency = truth == target_truth;

        if !consistency {
            commands.entity(sentence_entity).insert(BackgroundColorTransition::new(Color::srgb(1.0, 0.0, 0.0), Color::NONE, 1.0));
            all_consistent = false;
        }
    }

    if all_consistent {
        current_level.0 += 1;
    }
}

fn background_color_transition(
    mut commands: Commands,
    nodes: Query<(Entity, &mut BackgroundColorTransition, &mut BackgroundColor)>,
    time: Res<Time>,
) {
    let delta = time.delta();
    for (entity, mut transition, mut color) in nodes {
        if transition.timer.tick(delta).just_finished() {
            commands.entity(entity).remove::<BackgroundColorTransition>();
            *color = BackgroundColor(transition.end_color);
            continue;
        }

        let factor = transition.timer.fraction();
        let BackgroundColorTransition { start_color, end_color, .. } = *transition;
        let new_color = start_color.to_linear().mix(&end_color.to_linear(), factor).into();
        *color = BackgroundColor(new_color);
    }
}
