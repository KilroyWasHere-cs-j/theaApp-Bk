/*
* File for all system interactions
*
* Author: Gabriel Tower
* Date: 01-08-2025
*
*/


use chrono;

fn getSysTim() -> String {
    chrono::offset::Local::now()
} 
