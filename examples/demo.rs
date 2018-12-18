extern crate flow_rs;

use flow_rs::form::Form;
use flow_rs::proc_def::{ProcDef, Seq};
use flow_rs::proc_ins::ProcIns;
use flow_rs::task::{Task, TaskKind};

fn main() {
    let task1 = create_task1();
    let task2 = create_task2();
    let task3 = create_task3();
    let begin_task = Task::new("init".to_string(), TaskKind::BeginEvent, None, None);
    let end_task = Task::new("final".to_string(), TaskKind::EndEvent, None, None);
    let mut proc_def = ProcDef::new(
        "简单的测试流程".to_string(),
        "proc_definition_1".to_string(),
    );

    proc_def.set_seq(vec![
        Seq::new(&begin_task, &task1),
        Seq::new(&task1, &task2),
        Seq::new(&task2, &task3),
        Seq::new(&task3, &end_task),
    ]);

    let proc_ins = ProcIns::new(proc_def);
    proc_ins.run();
}

fn create_task1() -> Task {
    return Task::new(
        "任务1".to_string(),
        TaskKind::UserTask,
        Some(run_task1),
        None,
    );
}
fn run_task1(input: Option<Form>) {
    match input {
        Some(sf) => {
            println!("正在运行任务1, form_name={}", sf.name);
        }
        _ => {
            println!("正在运行任务1, 无输入");
        }
    }
}

fn create_task2() -> Task {
    return Task::new(
        "任务2".to_string(),
        TaskKind::UserTask,
        Some(run_task2),
        None,
    );
}
fn run_task2(input: Option<Form>) {
    match input {
        Some(sf) => {
            println!("正在运行任务2, form_name={}", sf.name);
        }
        _ => {
            println!("正在运行任务1, 无输入");
        }
    }
}

fn create_task3() -> Task {
    return Task::new(
        "任务3".to_string(),
        TaskKind::UserTask,
        Some(run_task3),
        None,
    );
}
fn run_task3(input: Option<Form>) {
    match input {
        Some(sf) => {
            println!("正在运行任务3, form_name={}", sf.name);
        }
        _ => {
            println!("正在运行任务1, 无输入");
        }
    }
}
