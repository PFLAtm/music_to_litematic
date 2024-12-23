use std::{borrow::Cow, collections::HashMap, env, process};


use fastnbt:: Value;
use mcdata::{mc1_18::{block_entity_compounds::ItemStack_save, block_entity_types::{self, BaseContainerBlockEntity, BlockEntity, RandomizableContainerBlockEntity}, props::Direction, BlockState}, util::BlockPos, GenericBlockEntity};
use rustmatica::Region;





fn main() {

    //prepare notes hashmap
    let mut notes: HashMap<&str, Vec<i8>> = HashMap::new();
    notes.insert("-",vec![0,0,0,0]);
    notes.insert("g",vec![1,0,0,0]);
    notes.insert("a",vec![0,1,0,0]);
    notes.insert("b",vec![1,1,0,0]);
    notes.insert("c",vec![0,0,1,0]);
    notes.insert("d",vec![1,0,1,0]);
    notes.insert("e",vec![0,1,1,0]);
    notes.insert("f",vec![1,1,1,0]);
    notes.insert("G",vec![0,0,0,1]);
    notes.insert("A",vec![1,0,0,1]);
    notes.insert("B",vec![0,1,0,1]);
    notes.insert("C",vec![1,1,0,1]);
    notes.insert("D",vec![0,0,1,1]);
    notes.insert("E",vec![1,0,1,1]);
    notes.insert("F",vec![0,1,1,1]);

    //prepare colors vec for easy access
    let color_vec = vec![BlockState::WhiteShulkerBox { facing: Direction::Up },BlockState::LightGrayShulkerBox { facing: Direction::Up },BlockState::GrayShulkerBox { facing: Direction::Up },BlockState::BlackShulkerBox { facing: Direction::Up }];


    //take input
    //let raw_input = "g c c d c b a a a d d e d c b g g e e f e d c a g g a d b c";
    
    let raw_input:Vec<String> = env::args().collect();

    //println!("{:#?}",raw_input);

    if raw_input[1] == "help" {
        println!("first arg: notes (seperated with whitespace), second arg: name of litematic file (without '.litematic') \n for more info ask PFLA");
        process::exit(0);
    }

    let mut filename = raw_input[2].clone();
    filename.push_str(".litematic");

    //let raw_input = "e e e - e e e - e G c d e - f f f f f e e e e e d e d G";
    let mut input = "- ".to_string();
    input.push_str(&raw_input[1]);
    //println!("{input}");
    
    //make input vector
    let mut max_rows = 1;

    let mut input_rows = Vec::new();
    let mut temp = String::new();
    for (i,c) in input.split_whitespace().enumerate() {
        if i % 28 == 0 && i!= 0{
            input_rows.push(temp.clone());
            temp = String::new();
            max_rows += 1;
        }
        temp.push_str(c);
    }

    /*if temp != "" {
        input_rows.push(temp);
    }*/

    //make all boxes full
    if temp != "" {
        while temp.len() <= 29 {
            temp.push_str("-");
        }
        input_rows.push(temp);
    }
    
    //println!("{:#?}",input_rows);

    

    //prepare region
    let mut region: Region<BlockState> = Region::new("music data", BlockPos::new(0, 0, 0), BlockPos::new(max_rows, 2, 4));
    
    
   
    
    for (current_row, row) in input_rows.iter().enumerate() {
        
        for i in 0..4 {

            //place shulker
            let pos = BlockPos::new(max_rows-current_row as i32 - 1 , 0, i);
            region.set_block(pos, BlockState::WhiteShulkerBox { facing: Direction::Up }.clone());
            

            //prepare items List
            let mut item_compounds = Vec::new();


            for (j,c) in row.chars().enumerate(){
                
            
                //prepare item hashmap
                let mut item = HashMap::new();

                //add the item
                item.insert("Slot".to_string(), Value::Byte(j as i8));
                item.insert("Count".to_string(), Value::Byte(1));
                
                if notes.get(c.to_string().as_str()).expect(format!("wrong: {c}").as_str())[i as usize] == 1{
                    item.insert("id".to_string(),Value::String("minecraft:wooden_shovel".to_string()));
                } else {
                    item.insert("id".to_string(),Value::String("minecraft:redstone".to_string()));
                }
                
                item_compounds.push(Value::Compound(item));
                
            }     

            
            //make nbt hasmap and add items map to it
            let mut nbt = HashMap::new();
            nbt.insert(Cow::Borrowed("Items"), Value::List(item_compounds)); 



            //place shulker Block Entity
            region.set_block_entity(GenericBlockEntity{id: Cow::Borrowed("ShulkerBox"),pos:BlockPos::new(max_rows-current_row as i32 - 1, 0, i),properties: nbt});

        }
    }

    
    for i in 0..4 {
        //place sign
        let pos = BlockPos::new(max_rows-1, 1, i);
        region.set_block(pos, BlockState::OakSign { rotation: bounded_integer::BoundedU8::new(12).unwrap(), waterlogged: false });

        //prepare text
        let mut text = "{\"text\":".to_string();
        text.push_str(format!("\"{}\"",i+1).as_str());
        text.push_str("}");
        //prepare nbt Hashmap
        let mut sign_nbt = HashMap::new();

        sign_nbt.insert(Cow::Borrowed("Color"),Value::String("black".to_string()));
        sign_nbt.insert(Cow::Borrowed("Text1"),Value::String(text));

        region.set_block_entity(GenericBlockEntity{id: Cow::Borrowed("minecraft:sign"), pos: pos, properties: sign_nbt});
    }
    //export file
    let file = region.as_litematic("music data", "PFLA");
    file.write_file(filename).unwrap();
}
