#![deny(unsafe_code)]
#![no_std]
#![no_main]



use cortex_m_rt::entry;
use rtt_target::{rtt_init_print, rprintln};
use panic_rtt_target as _;
use microbit::{
    board::Board,
    display::blocking::Display,
    hal::{Timer, prelude::*}
};

#[entry]

fn main() -> ! { 
    rtt_init_print!();

    let board = Board::take().unwrap();
    let mut timer = Timer::new(board.TIMER0);
    let mut display = Display::new(board.display_pins);

    let all_on = [
        [1, 1, 1, 1, 1],
        [1, 1, 1, 1, 1],
        [1, 1, 1, 1, 1],
        [1, 1, 1, 1, 1],
        [1, 1, 1, 1, 1]
    ];

    let mut active_state: [[u8; 5]; 5] = [
        [0, 0, 0, 0, 0], [0, 0, 0, 0, 0], [0, 0, 0, 0, 0], [0, 0, 0, 0, 0], [0, 0, 0, 0, 0]
    ];

    #[derive(Copy)]
    #[derive(Clone)]
    #[derive(Debug)]
    struct CellItem {
        active_row_num: usize,
        active_col_num: usize, 
        active_cell_state: i32, 
        is_active_yet: bool
    }
    
    let mut cell_display_state: [CellItem; 25] = [CellItem { active_row_num: 0, active_col_num: 0, active_cell_state: 0, is_active_yet: false}; 25];
    let mut next_active_col: usize = 0;
    let mut next_active_row: usize = 0;
    // let mut current_index: i64 = 0;

     loop {
        for current_index in 0..cell_display_state.len() {
            cell_display_state[current_index].active_row_num = (current_index as i64 / 4);
            cell_display_state[current_index].active_col_num = current_index % 4;
        }
        
        for current_index in 0..cell_display_state.len() {
            let row = cell_display_state[current_index].active_row_num;
            let col = cell_display_state[current_index].active_col_num;

            rprintln!("{:?}", (row, col));
        }
        timer.delay_ms(5_000_u32)
    }


    // for current_index in 0..cell_display_state.len() {
    //     let CellItem { active_row_num, active_col_num, active_cell_state, is_active_yet} = cell_display_state[current_index];

    //     // turn current cell off
    //     // cell_display_state[cell_display_state.len() - 1] = (active_row_num , active_col_num, 0); 
    //     // active_state[active_row_num][active_row_num] = 0;

    //     // set conditions for traversing the board cells
    //     if active_row_num == 0 {
    //         next_active_row = 0;
    //         next_active_col = next_active_col + 1;
            
    //         if active_col_num == 4 {
    //             next_active_row = active_row_num + 1;
    //             next_active_col = 4;
    //         }
    //     }

    //     if (active_row_num > 0 && active_row_num < 4) && active_col_num == 4 {
    //         next_active_row = active_row_num + 1;
    //         next_active_col = 4;
    //     }            

    //     if (active_row_num > 0 && active_row_num < 4) && active_col_num == 0 {
    //         next_active_row = active_row_num - 1;
    //         next_active_col = 0;
    //     }   

    //     if active_row_num == 4 {
    //         next_active_row = 4;
    //         next_active_col = active_col_num - 1;

    //         if active_col_num == 0 {
    //             next_active_row = active_row_num + 1;
    //             next_active_col = 0;
    //         }
    //     }      

    //     // append next cell and turn on
    //     cell_display_state[current_index] = (next_active_row, next_active_col, 1);
    //     active_state[next_active_row][next_active_col] = 1;

    // }

    // loop {
    //     if current_index < 1 {
    //         cell_display_state[0] = CellItem { active_row_num: 0, active_col_num: 0, active_cell_state: 0, is_active_yet: false };
    //         active_state[0][0] = 1;
    //     } else {
            
    //     }
        
    //     current_index = current_index += 1;
    //     display.show(&mut timer, active_state, 1000);
    //     timer.delay_ms(1_000_u32)
    // }
}
