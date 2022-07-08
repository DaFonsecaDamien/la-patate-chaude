/*pub struct MonstrousMaze {
    pub maze: Maze,
    pub player: Player,
    pub leaderboard: Leaderboard,
    pub challenge: Challenge,
    pub challenge_result: ChallengeResult,
}

impl MonstrousMaze {
    pub fn new(maze: Maze, player: Player, leaderboard: Leaderboard, challenge: Challenge, challenge_result: ChallengeResult) -> MonsterousMaze {
        MonsterousMaze {
            maze,
            player,
            leaderboard,
            challenge,
            challenge_result,
        }
    }

    fn find_start_and_end_of_maze(&self) -> (Point, Point) {
        let mut start_point = Point::new(0, 0);
        let mut end_point = Point::new(0, 0);
        for y in 0..self.maze.height {
            for x in 0..self.maze.width {
                if self.maze.get_cell(Point::new(x, y)).is_start() {
                    start_point = Point::new(x, y);
                }
                if self.maze.get_cell(Point::new(x, y)).is_end() {
                    end_point = Point::new(x, y);
                }
            }
        }
        return (start_point, end_point);
    }

    pub fn run(&mut self) {
        let (start_point, end_point) = self.find_start_and_end_of_maze();
        let mut current_point = start_point;
        let mut steps = 0;
        let mut used_time = 0.0;
        let mut is_finished = false;
        while !is_finished {
            let cell = self.maze.get_cell(current_point);
            if cell.is_end() {
                is_finished = true;
            } else {
                let next_point = self.find_next_point(current_point);
                if next_point.is_none() {
                    is_finished = true;
                } else {
                    current_point = next_point.unwrap();
                    steps += 1;
                    used_time += cell.get_used_time();
                }
            }
        }
        self.player.set_steps(steps);
        self.player.set_used_time(used_time);
    }
}*/