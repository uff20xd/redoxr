# Adding formless integration
The Idea is to modify main.rs before compilation by adding all the externs to it and cleaning them up afterwards

# Cargo compatibility
i extern the lib using --extern and then add -L release and -L release/deps
