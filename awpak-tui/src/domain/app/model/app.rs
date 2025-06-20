use awpak_tui_ai::domain::{agent::agent::AIAgent, chat::chat::Chat};

use crate::domain::{content_generator::model::content_generator::ContentGenerator, detail::model::detail::Detail, field::model::field::Field, input::model::input::Input, message::model::message::Message, movible::model::movible::MovibleAction, navigation::model::history::History, selectable::model::selectable_item::SelectableItem, sortable::model::sortable::SortBy, table::model::{row::Row, table::Table}};


#[derive(Clone)]
pub struct App
{
    sources : Table,
    sources_sort : SortBy,

    content : AppContent,
    content_sort : SortBy,
    content_search : Input,

    focus : AppFocus,

    content_generator : ContentGenerator,

    history : History,

    movible_action : MovibleAction,

    message : Option<Message>,

    field : Option<Field>,

    ai_agents : Vec<SelectableItem<AIAgent>>,

    saved_chats : Vec<SelectableItem<Chat>>
}

impl App
{
    pub fn new( sources : Table ) -> Self
    {
        Self 
        { 
            sources, 
            sources_sort : SortBy::Default,

            content : AppContent::Empty, 
            content_sort : SortBy::Default,
            content_search : Input::default(),

            focus : AppFocus::Sources, 

            content_generator : ContentGenerator::Empty,

            history : History::new(),

            movible_action : MovibleAction::None,

            message : None,

            field : None,

            ai_agents : vec![],

            saved_chats : vec![]
        }
    }

    pub fn sources( &self ) -> &Table
    {
        &self.sources
    }

    pub fn own_rows_sources( mut self ) -> ( Self, Vec<SelectableItem<Row>> )
    {
        let ( sources, rows ) = self.sources.own_rows();

        self.sources = sources;

        ( self, rows )
    }

    pub fn change_rows_sources( mut self, rows : Vec<SelectableItem<Row>> ) -> Self
    {
        self.sources = self.sources.change_rows( rows );

        self
    }

    // pub fn change_sources( mut self, new : Table ) -> App
    // {
    //     self.sources = new;

    //     self
    // }

    pub fn content( &self ) -> &AppContent
    {
        &self.content
    }

    pub fn own_content( mut self ) -> ( Self, AppContent )
    {
        let old = std::mem::replace( &mut self.content, AppContent::Empty );

        ( self, old )
    }

    pub fn own_detail_content( self ) -> ( Self, Option<Detail> )
    {
        let ( app, content ) = self.own_content();

        match content
        {
            AppContent::Table( t ) => ( app.change_content( AppContent::Table( t ) ), None ),
            AppContent::Empty => ( app, None ),
            AppContent::Detail( d ) => ( app, Some( *d ) ),
            AppContent::Chat( c ) => ( app.change_content( AppContent::Chat( c ) ), None )
        }
    }

    pub fn chat_content( &self ) -> Option<&Chat>
    {
        match self.content()
        {
            AppContent::Chat( c ) => Some( c ),
            _ => None    
        }
    }

    pub fn own_chat_content( self ) -> ( Self, Option<Chat> )
    {
        let ( app, content ) = self.own_content();

        match content
        {
            AppContent::Table( t ) => ( app.change_content( AppContent::Table( t ) ), None ),
            AppContent::Empty => ( app, None ),
            AppContent::Detail( d ) => ( app.change_content( AppContent::Detail( d ) ), None ),
            AppContent::Chat( c ) => ( app, Some( c ) )
        }
    }

    pub fn save_chat( mut self, chat : Chat ) -> Self
    {
        self.saved_chats.push( SelectableItem::Idle( chat ) );

        self
    }

    pub fn saved_chats( &self ) -> &Vec<SelectableItem<Chat>>
    {
        &self.saved_chats
    }

    pub fn own_saved_chats( mut self ) -> ( Self, Vec<SelectableItem<Chat>> )
    {
        let saved = std::mem::replace( &mut self.saved_chats, vec![] );

        ( self, saved )
    }

    pub fn change_saved_chats( mut self, new : Vec<SelectableItem<Chat>> ) -> Self
    {
        self.saved_chats = new;

        self
    }

    pub fn own_saved_chat( mut self, idx : usize ) -> ( Self, Option<Chat> )
    {
        if idx >= self.saved_chats.len() { return ( self, None ) }

        let ( _, chat ) = self.saved_chats.remove( idx ).own_inner();

        ( self, Some( chat ) )
    }

    pub fn change_content( mut self, new : AppContent ) -> App
    {
        self.content = new;

        self
    }

    pub fn content_generator( &self ) -> &ContentGenerator
    {
        &self.content_generator
    }

    pub fn own_content_generator( mut self ) -> ( Self, ContentGenerator )
    {
        let old = std::mem::replace( &mut self.content_generator, ContentGenerator::Empty );

        ( self, old )
    }

    pub fn detail_content_generator( &self ) -> Option<( &ContentGenerator, &String )>
    {
        match self.content_generator()
        {
            ContentGenerator::Directory( _ ) |
            ContentGenerator::Expandable( _ ) |
            ContentGenerator::ExecutableExpandable( _ ) |
            ContentGenerator::Chat( _, _ ) |
            ContentGenerator::Empty => None,
            ContentGenerator::Detail( parent, id ) => Some( ( parent, id ) )
        }
    }

    pub fn chat_content_generator( &self ) -> Option<( &ContentGenerator, &String )>
    {
        match self.content_generator()
        {
            ContentGenerator::Directory( _ ) |
            ContentGenerator::Expandable( _ ) |
            ContentGenerator::ExecutableExpandable( _ ) |
            ContentGenerator::Detail( _, _ ) |
            ContentGenerator::Empty => None,
            ContentGenerator::Chat( parent, id ) => Some( ( parent, id ) )
        }
    }

    pub fn change_content_generator( mut self, new : ContentGenerator ) -> Self
    {
        self.content_generator = new;

        self
    }

    pub fn history_next( &self ) -> Vec<&ContentGenerator>
    {
        self.history.next.iter().collect()
    }

    pub fn history_back( &self ) -> Vec<&ContentGenerator>
    {
        self.history.back.iter().collect()
    }

    pub fn own_history_next( mut self ) -> ( Self, Vec<ContentGenerator> )
    {
        let old = std::mem::replace( &mut self.history.next, vec![] );

        ( self, old )
    }

    pub fn own_history_back( mut self ) -> ( Self, Vec<ContentGenerator> )
    {
        let old = std::mem::replace( &mut self.history.back, vec![] );

        ( self, old )
    }

    pub fn change_history_next( mut self, new : Vec<ContentGenerator> ) -> Self
    {
        self.history.next = new;

        self
    }

    pub fn change_history_back( mut self, new : Vec<ContentGenerator> ) -> Self
    {
        self.history.back = new;

        self
    }

    pub fn focus( &self ) -> AppFocus
    {
        self.focus
    }

    pub fn change_focus( mut self, focus : AppFocus ) -> App
    {
        self.focus = focus;

        self
    }

    // pub fn maybe_change_focus( mut self, focus : Option<AppFocus> ) -> App
    // {
    //     if let Some( f ) = focus
    //     {
    //         self.focus = f;
    //     }

    //     self
    // }

    pub fn content_sort( &self ) -> SortBy
    {
        self.content_sort
    }

    pub fn change_content_sort( mut self, new : SortBy ) -> App
    {
        self.content_sort = new;

        self
    }

    pub fn sources_sort( &self ) -> SortBy
    {
        self.sources_sort
    }

    pub fn change_sources_sort( mut self, new : SortBy ) -> App
    {
        self.sources_sort = new;

        self
    }

    pub fn content_search( &self ) -> &Input
    {
        &self.content_search
    }

    pub fn own_content_search( mut self ) -> ( Self, Input )
    {
        let old = std::mem::replace( &mut self.content_search, Input::default() );

        ( self, old )
    }

    pub fn change_content_search( mut self, search : Input ) -> App
    {
        self.content_search = search;

        self
    }

    pub fn clone_content_search_text( &self ) -> String
    {
        self.content_search.text.clone()
    }

    pub fn movible_action( &self ) -> &MovibleAction
    {
        &self.movible_action
    }
    
    pub fn change_movible_action( mut self, new : MovibleAction ) -> Self
    {
        self.movible_action = new;

        self
    }

    pub fn message( &self ) -> Option<&Message>
    {
        self.message.as_ref()
    }

    pub fn change_message( mut self, new : Option<Message> ) -> Self
    {
        self.message = new;

        self
    }

    pub fn field( &self ) -> Option<&Field>
    {
        self.field.as_ref()
    }

    pub fn own_field( mut self ) -> ( Self, Option<Field> )
    {
        let f = self.field.take();

        ( self, f )
    }

    pub fn change_field( mut self, new : Option<Field> ) -> Self
    {
        self.field = new;

        self
    }

    pub fn ai_agents( &self ) -> &Vec<SelectableItem<AIAgent>>
    {
        &self.ai_agents
    }

    pub fn own_ai_agents( mut self ) -> ( Self, Vec<SelectableItem<AIAgent>> )
    {
        let old = std::mem::replace( &mut self.ai_agents, vec![] );

        ( self, old )
    }

    pub fn change_ai_agents( mut self, new : Vec<SelectableItem<AIAgent>> ) -> Self
    {
        self.ai_agents = new;

        self
    }

}

#[derive(Clone)]
pub enum AppContent
{
    Table( Table ),
    Detail( Box<Detail> ),
    Chat( Chat ),
    Empty
}

impl AppContent
{
    pub fn is_empty( &self ) -> bool
    {
        match self
        {
            AppContent::Empty => true,
            AppContent::Table( _ ) |
            AppContent::Detail( _ ) |
            AppContent::Chat { .. } => false    
        }
    }
}

// impl AppContent
// {
//     pub fn table( &self ) -> Option<&Table>
//     {
//         match self
//         {
//             AppContent::Table( t ) => Some( t ),
//             AppContent::Empty => None
//         }
//     }
// }

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum AppFocus
{
    Sources,
    Search,
    Content,
    Next,
    Up,
    Back,
    Confirm( Confirm ),
    Field
    // Cell
}

impl AppFocus
{
    pub fn is_confirm( &self ) -> bool
    {
        match self
        {
            Self::Confirm( _ ) => true,
            _ => false    
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Confirm
{
    MovibleAction,
    AgentSelection,
    ChatSelection
}

// pub enum AppResult
// {
//     Ok( App ),
//     Err( App, Error )
// }
