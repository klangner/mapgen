use mapgen::{
    MapBuilder,
    filter::{
        NoiseGenerator, 
        CellularAutomata,
        CullUnreachable,
        AreaStartingPosition,
        XStart, 
        YStart,
    },
};


fn main() {
    let map = MapBuilder::new(20, 20)
        .with(NoiseGenerator::uniform())
        .with(CellularAutomata::new())
        .with(AreaStartingPosition::new(XStart::CENTER, YStart::CENTER))
        .with(CullUnreachable::new())
        .build();  
    
        println!("{:}", &map);
}