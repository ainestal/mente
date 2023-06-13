use crate::openai::{chat_context::ChatContext, chat_gpt::Message};

pub struct Personalities {
    pub personalities: Vec<Personality>,
}

impl Personalities {
    pub fn new() -> Personalities {
        Personalities {
            personalities: Vec::new(),
        }
    }

    pub fn add_personality(&mut self, personality: Personality) {
        self.personalities.push(personality);
    }

    pub fn get_personality(&self, name: &str) -> Option<&Personality> {
        self.personalities.iter().find(|p| p.name == name)
    }

    pub fn get_personality_by_index(&self, index: usize) -> Option<Personality> {
        let p = self.personalities.get(index);
        match p {
            Some(p) => Some(Personality {
                name: p.name.clone(),
                prompt: p.prompt.clone(),
                context: p.context.clone(),
            }),
            None => None,
        }
    }
}

pub struct Personality {
    pub name: String,
    pub prompt: String,
    pub context: ChatContext,
}

impl Personality {
    pub fn new(name: String, prompt: String, model: String) -> Personality {
        let mut context = ChatContext::new(model);
        // Initialize the context of this Personality with the system prompt
        let message = Message {
            role: "system".to_string(),
            content: prompt.clone(),
        };
        context.push(message);

        Personality {
            name,
            prompt,
            context,
        }
    }

    pub fn push_to_context(&mut self, message: Message) {
        self.context.push(message);
    }
}
