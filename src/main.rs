mod stack;

use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::process::exit;

const EXIT: char = 'X';
const START: char = 'Y';
const MONSTER: char = 'M';
const UP: char = '^';
const DOWN: char = 'v';
const RIGHT: char = '>';
const LEFT: char = '<';

struct Informations{
    condition:Vec<char>,
    y_max:usize,
    x_max:usize,
    endurance:u8,
}

/// struct pour la case d'entrée
struct Start {
    x: usize,
    y: usize,
}

/// struct pour savoir ou est le joueur
struct Cell {
    x: usize,
    y: usize,
}

pub struct MonstrousMazeInput {
    pub grid: String,
    pub endurance: u8,
}

pub struct MonstrousMazeOutput {
    pub path: String,
}

/// déplacement vers le haut
fn up(current_cell: &mut Cell, tableau_save:&mut Vec<Vec<usize>>){
    tableau_save[current_cell.x][current_cell.y] = 1 ;
    current_cell.x = current_cell.x-1;
    tableau_save[current_cell.x][current_cell.y]  = 2 ;
}
fn right(current_cell: &mut Cell, tableau_save:&mut Vec<Vec<usize>>){
    tableau_save[current_cell.x][current_cell.y] = 1 ;
    current_cell.y = current_cell.y+1;
    tableau_save[current_cell.x][current_cell.y]  = 2 ;
}
fn down(current_cell: &mut Cell, tableau_save:&mut Vec<Vec<usize>>){
    tableau_save[current_cell.x][current_cell.y] = 1 ;
    current_cell.x = current_cell.x+1;
    tableau_save[current_cell.x][current_cell.y]  = 2 ;
}
fn left(current_cell: &mut Cell, tableau_save:&mut Vec<Vec<usize>>){
    tableau_save[current_cell.x][current_cell.y] = 1 ;
    current_cell.y = current_cell.y-1;
    tableau_save[current_cell.x][current_cell.y]  = 2 ;
}

fn is_up_reachable(current_cell: &Cell, tableau_complet: &Vec<Vec<char>>, tableau_save:&Vec<Vec<usize>>, infos:&Informations) -> bool {
    return if current_cell.x > 0 {
        let charact = tableau_complet[current_cell.x-1][current_cell.y];
        if infos.condition.contains(&charact) && (tableau_save[current_cell.x-1][current_cell.y] != 1) {
            true
        }else{
            false
        }
    } else {
        false
    }
}

fn is_right_reachable(current_cell:&Cell, tableau_complet:&Vec<Vec<char>>, tableau_save:&Vec<Vec<usize>>, infos:&Informations) -> bool {
    //pas sur pour le tabsave
    return if current_cell.y+1 < infos.y_max {
        let charact = tableau_complet[current_cell.x][current_cell.y+1];
        if infos.condition.contains(&charact) && (tableau_save[current_cell.x ][current_cell.y+ 1] != 1) {
            true
        }else{
            false
        }
    } else {
        false
    }
}

fn is_down_reachable(current_cell:&Cell, tableau_complet:&Vec<Vec<char>>, tableau_save:&Vec<Vec<usize>>, infos:&Informations) -> bool {
    //pas sur pour le tabsave
    return if current_cell.x+1 < infos.x_max {
        let charact = tableau_complet[current_cell.x+1 ][current_cell.y];
        if infos.condition.contains(&charact) && (tableau_save[current_cell.x +1][current_cell.y] != 1) {
            true
        }else{
            false
        }
    } else {
        false
    }
}

fn is_left_reachable(current_cell:&Cell, tableau_complet:&Vec<Vec<char>>, tableau_save:&Vec<Vec<usize>>, infos: &Informations) -> bool {
    //pas sur pour le tabsave
    return if current_cell.y-1 > 0 {
        let charact = tableau_complet[current_cell.x ][current_cell.y-1];
        if infos.condition.contains(&charact) && (tableau_save[current_cell.x][current_cell.y-1] != 1) {
            true
        }else{
            false
        }
    } else {
        false
    }
}

fn endurance_decr(current_cell:&Cell, tableau_complet:&Vec<Vec<char>>, mut infos: &mut Informations) -> bool{
    return if tableau_complet[current_cell.x][current_cell.y] == infos.condition[0] {
        infos.endurance -= 1;
        println!("Monstre : x = {} | y = {}", current_cell.x, current_cell.y-1);
        if infos.endurance == 0 {
            false
        } else {
            true
        }
    } else {
        true
    }
}

fn success(current_cell:&Cell, tableau_complet:&Vec<Vec<char>>) -> bool {
    return if tableau_complet[current_cell.x][current_cell.y] == EXIT {
        true
    } else {
        false
    }
}



fn main() {
    let mut maze = "│Y────┬─────────┬─┬─────┬───┬───────┬───────────┬─────────────┬───┬─┬─┬─────┬───┬─┬─┬───┬─────┬───────────┬─────┬─┬───┬───┬─────┬───────────────────┬───┬─┬─┬─┬─┬─┬───┬───┬─┬─┬───┬───────────┬─┬───┬───┐
│     │         │ │     │   │       │           │             │   │ │ │     │   │ │ │   │     │           │     │ │   │   │     │                   │   │ │ │ │ │ │   │   │ │ │   │           │ │   │   │
├── ──┴─┬── ────┘ │ ────┼─┐ ├── ──┬─┴─┬─┐ ──┬── │ │ ──┬──── ──┘ ┌─┘ │ └── ──┼── │ │ ├── ├── ──┘ ┌─┐ ──┬── │ ┌── │ │ ──┘ │ │ ────┼─────┐ ┌──────── ──┴── │ │ │ │ │ │ ┌─┴── │ │ ├── │ ┌──── ┌── │ │ ──┘ ──┤
│       │               │ │ │     │   │ │   │     │   │         │           │       │   │       │ │   │   │ │   │       │       │     │ │                   │     │ │         │     │     │     │       │
│ ┌─┬───┴─┬── │ ┌── ┌── │ │ └─┐ ──┼── │ ├── ├─┐ │ │ ┌─┴─┬─┬─┐ ──┴─────┬─┬─┐ └── ┌───┘ ┌─┴───┬─┐ │ ├─┐ └─┬─┼─┘ │ │ ┌─┐ ┌─┘ ────┬─┘ ┌───┴─┘ ┌──── ──┐ ────┐ ┌─┤ ┌─┬─┘ │ │ ┌─────┴── ──┴─┬── │ ──┬─┼── ────┤
│ │ │     │   │ │   │   │     │   │   │ │   │ │ │ │ │   │ │ │         │ │ │     │     │     │ │   │ │   │ │   │   │ │ │       │   │       │       │     │ │ │ │ │     │ │             │   │   │ │       │
│ │ │ ──┐ ├───┤ └─┬─┤ │ ├── │ │ │ │ ──┘ ├── │M└─┘ ├─┴── │ │ │ │ │ ──┐ │ │ │ ┌── └──── ├───┐ │ │ │ │ └───┘ ├── ├─┬─┘ │ ├─┬── ──┤ │ │ ──┐ ──┤ ────┐ │ │ │ │ │ │ │ │ ────┼─┼──── │ ┌─┐ │ ├───┘ │ │ │ │ ┌── │
│   │   │ │   │   │ │ │ │   │   │ │     │   │     │           │ │   │ │     │         │   │     │ │       │   │ │     │ │     │ │     │   │     │ │ │ │ │ │           │ │     │ │ │ │ │     │   │ │ │   │
│ ──┘ │ │ │ ──┴─┐ │ └─┘ │ ┌─┘ ┌─┘ │ ──┐ │ │ ├───┬─┴─┬── │ │ │ ├─┘ ┌─┘ │ │ │ └─┬── ┌───┘ ┌─┴─┐ │ ├─┤ ┌── ┌─┴─┐ │ │ ┌─┐ │ ├─┐ ┌─┴─┴─────┴── ├─────┤ ├─┤ │ └─┼─┬── ┌──── │ │ ────┤ │ └─┴─┴─┬───┴───┴─┤ └───┤
│     │ │       │ │       │   │   │   │ │ │ │   │   │   │ │ │ │   │     │ │   │   │     │   │ │ │ │ │   │   │ │   │ │ │ │ │ │             │     │ │ │ │   │ │   │       │     │         │         │     │
├── ┌─┴─┴─┐ ──┐ └─┴─┐ │ ──┼─┬─┴── │ ──┼─┴─┼─┘ ──┤ ┌─┤ ┌─┼─┴─┘ └───┤ ──┐ └─┼───┘ │ │ ┌───┤ ┌─┘ │ │ └─┴── │ ┌─┼─┤ ──┘ │ │ │ └─┼───┐ ┌─┬── ──┘ │ │ ├─┤ └─┼───┘ └─┐ └─┬─┐ ┌─┘ │ ──┤ ──┬─┐ ┌─┘ ──────┬─┼─┐ │ │
│   │     │   │     │ │   │ │         │   │     │ │ │ │ │         │   │   │     │   │   │ │   │           │ │ │     │       │   │ │ │       │ │ │ │   │       │   │ │ │   │   │   │ │ │         │ │ │ │ │
│ │ └── ┌─┴───┴── ──┘ │ ──┤ │ ──┬─┐ ──┘ ──┴── │ │ │ └─┤ └── ──┐ ┌─┼───┤ ┌─┴──── │ │ │ │ │ ├─┐ ├── ──┬───┬─┘ │ │ ┌─┐ ├── ──┐ │ ──┘ │ └───┐ │ └─┤ │ ├── │ ──┐ ──┴─┬─┘ │ │ ┌─┤ ┌─┘ ──┘ │ │ │ ┌── │ │ │ └─┘ │
│ │     │             │   │     │ │           │       │       │ │ │   │ │       │ │ │ │   │ │ │     │   │   │   │ │ │     │             │ │   │   │   │   │     │     │ │ │ │       │   │ │   │ │       │
├─┤ ────┤ ┌──── │ ──┐ │ ──┼── ┌─┘ └───┐ ┌─┐ │ ├──── │ ├─┐ ┌── ├─┘ │ │ │ └───┐ ┌─┴─┴─┘ │ ──┘ └─┤ ──┐ └─┐ │ │ ├───┘ │ ├─┬───┴─┬── ──┐ │ │ ├─┤ ──┴───┘ │ └─┬─┘ ──┐ ├─┐ ┌─┼─┘ │ ├───┐ ┌─┘ ┌─┴─┤ ┌─┼─┴───┬── │
│ │     │ │     │   │ │   │   │       │ │ │ │ │     │ │ │ │   │     │ │     │ │       │       │   │   │   │ │       │ │     │     │ │ │ │ │         │   │     │ │ │ │ │   │ │   │ │   │   │ │ │     │   │
│ └─┐ ┌─┤ ├──── │ │ │ └─┐ ├─┐ │ ┌───┬─┘ │ └─┼─┴──── │ │ │ ├── ├───┐ └─┘ │ ──┼─┘ ┌─┬── ├───┐ ┌─┘ ┌─┴── └── └─┴─┐ ──┐ │ ├── │ └── │ ├─┴─┘ │ ├─┬───┬───┼───┤ ┌── └─┘ │ │ └─┐ │ │ │ ├─┤ ┌─┼─┐ │ │ │ ──┐ │ │ │
│   │ │ │ │     │ │ │   │ │ │   │   │   │   │       │ │   │   │   │     │   │   │ │   │   │ │   │             │   │   │   │     │ │       │ │   │   │   │ │         │   │ │   │ │ │ │ │ │   │ │   │   │ │
├─┐ │ │ └─┤ ┌───┤ └─┼─┬─┼─┤ │ │ │ │ └─┬─┘ │ │M┌── ──┤ │ ──┴─┬─┴── │ │ ┌─┼───┴───┘ │ ──┘ ──┤ ├─┬─┴───┬─┐ ──┬─┬─┤ ┌─┼── │ ┌─┴───┬─┘ │ ┌─┬── │ │ ──┘ │ │ │ └─┴── ──┬─┐ │ ──┤ │ │ ├─┘ └─┘ │ ├───┘ │ │ │ │ └─┤
│ │ │     │ │   │   │ │ │ │   │   │   │   │   │     │       │     │ │ │ │         │       │ │ │     │ │   │ │ │ │ │   │ │     │   │ │ │           │ │ │         │ │     │   │ │         │       │ │ │   │
│ │ │ ┌── │ └─┐ │ ──┘ │ │ ├── ├───┴─┬─┼── ├─┐ ├───┬─┴─┐ │ ┌─┤ │ ──┼─┤ │ │ │ ┌─────┼───┬───┤ │ └─┬── │ ├── │ │ └─┘ └───┘ ├───┐ └─┬─┴─┘ └── ────┐ │ │ └─┴── ──┐ ┌─┘ │ ──┬─┘ ──┼─┤ ──┐ │ ──┼── ──┬─┘ ├─┴───┤
│   │ │   │   │       │   │   │     │ │   │ │ │   │   │ │ │ │ │   │ │   │ │ │     │   │   │ │   │     │   │             │   │   │             │ │ │         │ │   │   │     │ │   │ │   │     │   │     │
├─┐ └─┼── └───┼─┬── │ │ ──┘ ┌─┘ ┌─┬─┤ └───┤ └─┴─┐ └─┐ │ └─┘ └─┤ ┌─┘ │ ──┤ └─┴──── │ ──┤ │ │ │ │ └─┬── ├── ├─┬─┬─┐ ┌─┐ ┌─┤ ──┤ ┌─┘ │ │ ────────┤ └─┴─┐ ──┐ ──┴─┤ ──┼── └── ──┘ ├───┘ │ ──┘ ────┴─┬─┤ │ │ │
│ │   │       │ │   │ │     │   │ │ │     │     │   │ │       │ │       │             │ │     │   │   │   │ │ │ │ │ │ │ │   │ │   │ │         │     │   │     │   │           │     │           │ │ │ │ │
│ │ │ ├── │ ┌─┘ ├───┘ │ ────┘ │ │ │ │ ┌─┐ │ │ │ │ ──┤ └─┐ ┌── │ │ │ ┌── ├─┬── │ ┌─┬───┘ ├──── ├── ├── │ ┌─┤ │ │ │ │ ├─┘ └── │ └───┼─┘ │ ────┬─┼─┬───┴── ├── ┌─┤ │ │ │ │ │ ┌─┐ └──── └───┐ │ ──┬─┘ │ │ └─┤
│   │ │   │ │   │     │       │ │   │ │ │ │ │ │     │   │ │     │ │ │   │ │   │ │ │     │     │   │     │ │         │       │     │   │     │ │ │       │   │ │ │ │ │ │ │ │ │           │ │   │     │   │
│ │ └─┴───┘ │ │ └─┐ ──┤ ┌─┐ ──┴─┼── ├─┘ │ │ └─┼──── │ │ └─┴─┬─┬─┴─┴─┘ ──┤ │ ┌─┼─┘ └─┐ ──┤ │ │ │ │ ├──── │ │ ──┐ ──┬─┘ ┌── ──┘ │ ┌─┘ ──┴─┐ ┌─┘ │ │ ──┬─┐ ├───┘ └─┘ │ ├─┤ └─┘ │ ┌── ──┐ ┌─┴─┘ ──┼── ──┤ ──┤
│ │           │   │   │ │ │     │   │   │ │   │       │     │ │         │   │ │     │   │ │ │ │ │ │           │   │   │       │ │       │ │     │   │ │ │           │ │     │ │     │ │       │     │   │
├─┘ ──────┬───┤ ──┴───┤ │ │ │ ┌─┘ ──┴── │ │ ──┴─────┐ ├─┬── │ │ ──┬── │ ├── │ └───┐ │ ──┤ ├─┤ │ │ └── │ ──┐ ┌─┴── └─┐ │ ┌───┐ └─┼── ────┴─┴───┐ └── │ ├─┘ ──┬───┐ │ │ │ ──┐ │ │ │ │ │ ├─┐ ──┬─┴─┬─┐ │ ──┤
│         │   │       │ │   │ │         │           │ │ │   │ │   │   │ │   │     │     │ │ │ │ │     │   │ │       │ │ │   │   │             │       │     │   │ │   │   │ │ │ │ │ │ │ │   │   │ │ │   │
│ │ ┌─┐ ──┘ ┌─┘ │ ┌─┬─┴─┼───┘ └── │ ┌───┼──── ──┬── ├─┤ └───┘ ├── ├─┐ │ │ ──┼── │ └──── │ │ ├─┴─┼───┐ │ ┌─┼─┴───┬─┐ ├─┴─┘ ┌─┼─┐ │ │ ──┬── ┌─┐ ├── │ ──┤ ──┐ ├─┐ │ ├───┤ ┌─┴─┴─┴─┤ ├─┴─┤ │ ──┘ │ │ │ └─┬─┤
│ │ │ │     │   │ │ │   │         │ │   │       │   │ │       │   │ │ │     │   │       │   │   │   │ │ │ │     │ │ │     │ │ │   │   │   │ │ │   │   │   │ │ │   │   │ │       │ │   │ │     │ │     │ │
├─┼─┤ ├─┬── ├───┘ │ └─┐ └── ──┐ ┌─┘ │ ──┘ ┌─────┘ ┌─┘ │ │ ────┤ ──┘ ├─┼── ┌─┴─┬─┼─┐ ──┐ └─┬─┘ ┌─┤ ──┴─┤ │ └─┬─┐ │ │ ├───┐ │ │ └─┐ └─┐ ├───┘ │ ├───┼───┘ │ ├─┘ │ │ │ ──┴─┤ ┌───┐ │ │ ──┤ │ │ ──┤ └─┐ ──┘ │
│ │ │ │ │   │         │       │ │         │       │   │ │     │     │ │   │   │ │ │   │   │   │ │     │     │ │     │   │ │     │   │ │       │   │     │ │     │ │     │ │   │       │ │ │   │   │     │
│ │ │ │ │ ┌─┴───┐ │ ──┤ ┌─────┴─┘ ──┬───┐ ├── ┌── │ │ ├─┘ ┌───┘ │ ┌─┘ ├── │ │ │ │ └── ├─┐ ├── │ │ ────┘ ┌───┘ └─────┴── │ └── ──┤ ┌─┘ ├───┐ ──┘ │ ├── │ ├─┼── ──┼─┼─┐ │ ├─┘ ──┴─────┐ │ │ │ ──┤ │ │ │ │ │
│     │ │ │     │ │   │ │           │   │ │   │   │ │ │   │     │ │   │   │ │         │ │ │             │                       │ │   │   │     │ │   │ │ │     │ │ │ │ │           │ │   │   │ │   │ │ │
│ │ │ │ │ │ ──┐ ├─┘ ──┼─┤ │ ┌── ┌── │ │ ├─┴───┘ ──┼─┘ ├── └── ┌─┘ │ │ │ │ │ │ │ ──┐ ──┤ │ └─────┐ │ │ │ └── ────┬───┐ ┌── ────┬─┴─┤ ──┘ ──┤ │ ──┼─┘ │ └─┘ ├───┬─┘ │ │ ├─┼── ┌──── ┌─┴─┘ │ ├── └─┴─┐ ├─┤ │
│ │ │     │   │ │     │ │ │ │   │   │ │ │         │   │       │   │ │   │   │ │   │   │         │ │ │ │         │   │ │       │   │       │ │   │   │     │   │   │   │ │   │     │     │ │       │ │ │ │
├─┴─┘ ────┼───┘ └──── │ │ │ │ ┌─┘ │ │ └─┴─────┐ │ └── │ ┌── │ ├── └─┤ ──┤ ──┴─┴─┐ ├── │ │ ──────┼─┘ └─┼── ┌──── └─┐ │ │ ──┬── └── ├── ┌───┘ │ │ │ ┌─┘ ────┘ │ │ │ │ ──┘ │ │ ├── │ │ │ ──┼─┴── ────┤ │ │ │
│         │             │ │ │ │   │           │ │     │ │   │ │     │   │       │ │   │ │       │     │   │       │   │   │       │   │     │ │ │ │         │   │ │       │ │   │   │   │         │ │ M X
└─────────┴─────────────┴─┴─┴─┴───┴───────────┴─┴─────┴─┴───┴─┴─────┴───┴───────┴─┴───┴─┴───────┴─────┴───┴───────┴───┴───┴───────┴───┴─────┴─┴─┴─┴─────────┴───┴─┴───────┴─┴───┴───┴───┴─────────┴─┴────";
    let mut endu = 3;

    let mut input = MonstrousMazeInput {
        grid: maze.to_string(),
        endurance: endu,
    };
    start(input);
}

fn start(monster_struct:MonstrousMazeInput) -> MonstrousMazeOutput{
    // init
    let mut dpt = Start {
        x: 0,
        y: 0,
    };
    let mut joueur = Cell {
        x: 0,
        y: 0,
    };

    let mut infos = Informations{
        condition : Vec::from(['M', ' ', EXIT, START]),
        y_max: 0,
        x_max: 0,
        endurance: 3,
    };

    // flag pour savoir si on a bien trouvé le caractère de début
    let mut flag = 0;
    let mut abs = 0;
    let mut tableau_complet = Vec::new();
    let mut tableau_save = Vec::new();

    // lecture de fichier
   /* let file = match File::open("src/maze1.txt") {
        Ok(file) => file,
        Err(err) => {
            println!("Erreur lors de l'ouverture du fichier: {}", err);
            exit(0);
        }
    };

    // analyse de fichier + création des 2 tableaux
    let reader = BufReader::new(file);
*/
    for row in monster_struct.grid.split('\n'){
        println!("{}", row);
        let mut ligne_tableau = Vec::new(); //tableau identique au fichier
        let mut ligne_secondaire = Vec::new(); //tableau de 1 0 -1 pour save ou on est passé
        let mut ord = 0; //ordonnée pour savoir ce que l'on lit

        //divise la ligne en charactères exploitable
        for charact in row.chars(){

            //attribuer un chiffre à un charactère dans le tableau de sauvegarde
            match charact{
                START => {
                    dpt.x = abs;
                    dpt.y =  ord;
                    joueur.x = abs ;
                    joueur.y = ord ;
                    ligne_secondaire.push(2);
                    flag = 1;
                },
                MONSTER => {
                    ligne_secondaire.push(4);
                },
                EXIT => {
                    ligne_secondaire.push(3);
                },
                ' '=> {
                    ligne_secondaire.push(0);
                },
                _=> ligne_secondaire.push(5)
            };

            ligne_tableau.push(charact);
            ord+=1;
        }
        tableau_save.push(ligne_secondaire);
        tableau_complet.push(ligne_tableau);
        abs+=1;
    }
/*
    for line in reader.lines() {
        let line = match line {
            Ok(line) => line,
            Err(err) => {
                println!("Erreur lors de la lecture d'une ligne: {}", err);
                return;
            }
        };
        // arreter au saut de ligne
        let spli:String = line.split('\n').collect();

        //definitions de valeurs
        let mut ligne_tableau = Vec::new(); //tableau identique au fichier
        let mut ligne_secondaire = Vec::new(); //tableau de 1 0 -1 pour save ou on est passé
        let mut ord = 0; //ordonnée pour savoir ce que l'on lit

        //divise la ligne en charactères exploitable
        for charact in spli.chars(){

            //attribuer un chiffre à un charactère dans le tableau de sauvegarde
            match charact{
                'Y' => {
                    dpt.x = abs;
                    dpt.y =  ord;
                    joueur.x = abs ;
                    joueur.y = ord ;
                    ligne_secondaire.push(2);
                    flag = 1;
                },
                'M'=> {
                    ligne_secondaire.push(4);
                },
                'X' => {
                    ligne_secondaire.push(3);
                },
                ' '=> {
                    ligne_secondaire.push(0);
                },
                _=> ligne_secondaire.push(5)
            };

            ligne_tableau.push(charact);
            ord+=1;
        }
        tableau_save.push(ligne_secondaire);
        tableau_complet.push(ligne_tableau);
        abs+=1;
    }

 */

    //pas de point de départ
    if flag != 1{
        exit(0);
    }

    //Def variable globale
    infos.y_max = tableau_complet[0].len() ;
    infos.x_max = tableau_complet.len()  ;

    println!("Taille du tableau : {:?} x {}", infos.x_max,infos.y_max);


    let mut pile = stack::Stack::new();
    let mut path = stack::Stack::new();


    let mut tour = 0;
    let mut flag_fin = false;

    while !flag_fin {

        tour +=1;

        if is_up_reachable(&joueur, &tableau_complet, &tableau_save, &infos){
            //println!("haut");
            pile.push(Cell { x: joueur.x, y: joueur.y });
            path.push(UP);
            up(&mut joueur, &mut tableau_save);
            if !endurance_decr(&joueur, &tableau_complet, &mut infos){
                println!("plus d'endurance");
                flag_fin = true;
            }
            if success(&joueur, &tableau_complet){
                println!("SUCCCESSSSS");
                flag_fin = true;
            }
        }else{
            if is_right_reachable(&joueur, &tableau_complet, &tableau_save, &infos){
                //println!("droite");
                pile.push(Cell { x: joueur.x, y: joueur.y });
                path.push(RIGHT);
                right(&mut joueur, &mut tableau_save);
                if !endurance_decr(&joueur, &tableau_complet, &mut infos){
                    println!("plus d'endurance");
                    flag_fin = true;
                }
                if success(&joueur, &tableau_complet){
                    println!("SUCCCESSSSS");
                    flag_fin = true;
                }
            }else{
                if is_down_reachable(&joueur, &tableau_complet, &tableau_save, &infos){
                    //println!("bas");
                    pile.push(Cell { x: joueur.x, y: joueur.y });
                    path.push(DOWN);
                    down(&mut joueur, &mut tableau_save);
                    if !endurance_decr(&joueur, &tableau_complet, &mut infos){
                        println!("plus d'endurance");
                        flag_fin = true;
                    }
                    if success(&joueur, &tableau_complet){
                        println!("SUCCCESSSSS");
                        flag_fin = true;
                    }
                }else{
                    if is_left_reachable(&joueur, &tableau_complet, &tableau_save, &infos){
                        //println!("gauche");
                        pile.push(Cell { x: joueur.x, y: joueur.y });
                        path.push(LEFT);
                        left(&mut joueur, &mut tableau_save);
                        if !endurance_decr(&joueur, &tableau_complet, &mut infos){
                            println!("plus d'endurance");
                            flag_fin = true;
                        }

                        if success(&joueur, &tableau_complet){
                            println!("SUCCCESSSSS");
                            flag_fin = true;
                        }
                    }else{
                        //println!("no direction");
                        if let Some(ancien) = pile.pop(){
                            //println!("ancien {} {} nouveau {} {}", ancien.x, ancien.y,joueur.x, joueur.y );
                            tableau_save[joueur.x][joueur.y] = 1 ;
                            joueur.x = ancien.x;
                            joueur.y = ancien.y;
                            tableau_save[joueur.x][joueur.y]  = 2 ;
                        }
                        path.pop();
                    }
                }
            }
        }
        //println!("Position : {} {}", joueur.x, joueur.y);
        //println!("-----------------------");
    }
    for elemnt in &pile.elements {
        tableau_complet[elemnt.x][elemnt.y] = '#';
    }

    println!("Nombre de coups : {} ", tour);
    println!("Endurance :  {} ", infos.endurance);



    // write(&tableau_complet);

    let mut str  =  String::new();

    str = write_path_to_string(path);

    let mut output = MonstrousMazeOutput{
        path: str.to_string(),
    };

    return output;

}
/*
fn write(tableau_complet:&Vec<Vec<char>>) -> std::io::Result<()> {
    let mut file = File::create("maze_res.txt")?;
    let data = tableau_complet.iter().map(|row| row.iter().collect::<String>()).collect::<Vec<String>>().join("\n");

    file.write_all(data.as_bytes())?;
    Ok(())
}
*/
fn write_path_to_string(path_pile:stack::Stack<char>) -> String {
    let mut str  =  String::new();
    for element in path_pile.elements{
        str.push(element);
    }
    return str;
}
