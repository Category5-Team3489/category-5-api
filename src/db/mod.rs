pub mod data;

pub mod crud;

use std::sync::Arc;

use axum::Extension;
use chrono::{DateTime, Utc};
use tokio::sync::{Notify, OnceCell};
use tokio::task::JoinHandle;

use self::crud::Crud;
use self::data::DbData;
use self::data::event::Event;
use self::data::student::Student;

pub struct Db {
    rx: flume::Receiver<DbFunction>,
    data: DbData,
}

impl Db {
    pub fn new(rx: flume::Receiver<DbFunction>) -> Self {
        Self {
            rx,
            data: DbData::new(),
        }
    }

    pub fn start(mut self) -> JoinHandle<()> {
        tokio::spawn(async move {
            while let Ok((notifier, input, output)) = self.rx.recv_async().await {
                let o = match input {
                    DbInput::Ping => {
                        DbOutput::Pong
                    }

                    // data
                    DbInput::Clone => {
                        DbOutput::Clone(self.data.clone())
                    }

                    // data.students
                    DbInput::CreateStudent(name, should_read) => {
                        let o = Student::create(&mut self.data.students, Student::new(name), should_read);
                        DbOutput::CreateStudent(o)
                    }
                    DbInput::ReadStudent(predicate) => {
                        let o = Student::read(&self.data.students, predicate);
                        DbOutput::ReadStudent(o)
                    }
                    DbInput::UpdateStudent(predicate, update, should_read) => {
                        let o = Student::update(&mut self.data.students, predicate, update, should_read);
                        DbOutput::UpdateStudent(o)
                    }
                    DbInput::DeleteStudent(predicate) => {
                        let o = Student::delete(&mut self.data.students, predicate);
                        DbOutput::DeleteStudent(o)
                    }

                    // data.events
                    DbInput::CreateEvent(name, kind, start_time, end_time, should_read) => {
                        let o = Event::create(&mut self.data.events, Event::new(name, kind, start_time, end_time), should_read);
                        DbOutput::CreateEvent(o)
                    }
                    DbInput::ReadEvent(predicate) => {
                        let o = Event::read(&self.data.events, predicate);
                        DbOutput::ReadEvent(o)
                    }
                    DbInput::UpdateEvent(predicate, update, should_read) => {
                        let o = Event::update(&mut self.data.events, predicate, update, should_read);
                        DbOutput::UpdateEvent(o)
                    }
                    DbInput::DeleteEvent(predicate) => {
                        let o = Event::delete(&mut self.data.events, predicate);
                        DbOutput::DeleteEvent(o)
                    }
                };
                if let Err(err) = output.set(o) {
                    println!("Error: {:?}", err);
                }
                drop(output); // because of Arc::try_unwrap on response
                notifier.notify_one();
            }
        })
    }
}

impl Db {
    pub async fn ext_call(db: Extension<DbConnection>, input: DbInput) -> DbOutput {
        Self::req(db.0, input).await
    }
    
    pub async fn call(db: DbConnection, input: DbInput) -> DbOutput {
        let notifier = Arc::new(Notify::new());
        let output = Arc::new(OnceCell::new());
    
        db.send((notifier.clone(), input, output.clone()))
            .unwrap();
        notifier.notified().await;
    
        // https://stackoverflow.com/questions/29177449/how-to-take-ownership-of-t-from-arcmutext
        Arc::try_unwrap(output).unwrap().take().unwrap()
    }

}

#[allow(dead_code)]
pub enum DbInput {
	Ping,

    // data
    Clone,

    // data.students
    CreateStudent(String, bool),
    ReadStudent(Box<dyn FnMut(&Student) -> bool + Send>),
    UpdateStudent(Box<dyn FnMut(&Student) -> bool + Send>, Box<dyn FnMut(&mut Student) + Send>, bool),
    DeleteStudent(Box<dyn FnMut(&Student) -> bool + Send>),

    // data.events
    CreateEvent(String, String, DateTime<Utc>, DateTime<Utc>, bool),
    ReadEvent(Box<dyn FnMut(&Event) -> bool + Send>),
    UpdateEvent(Box<dyn FnMut(&Event) -> bool + Send>, Box<dyn FnMut(&mut Event) + Send>, bool),
    DeleteEvent(Box<dyn FnMut(&Event) -> bool + Send>),
}

#[allow(dead_code)]
#[derive(Debug)]
pub enum DbOutput {
	Pong,
    Void,

    // data
    Clone(DbData),

    // data.students
    CreateStudent(Option<Student>),
    ReadStudent(Option<Student>),
    UpdateStudent(Option<Student>),
    DeleteStudent(usize),

    // data.events
    CreateEvent(Option<Event>),
    ReadEvent(Option<Event>),
    UpdateEvent(Option<Event>),
    DeleteEvent(usize),
}

pub type DbFunction = (Arc<Notify>, DbInput, Arc<OnceCell<DbOutput>>);

pub type DbConnection = Arc<flume::Sender<DbFunction>>;
