export interface Rom {
    filename: string;
    label: string;  
    description: string;
    controls: { key: string; action: string }[];
    tips: string[];
    objective: string;
    speed: number;
}

export const ROMS: Rom[] = [
    {
        filename: 'breakout',
        label: 'Breakout',
        description: 'The classic arcade game where you control a paddle to bounce a ball off the top of the screen and break bricks.',
        controls: [
            { key: 'Q', action: 'Move paddle left' },
            { key: 'E', action: 'Move paddle right' },
        ],
        tips: ['Use the paddle to bounce the ball off the top of the screen', 'Break all the bricks to win'],
        objective: 'Break all the bricks to win',
        speed: 10
    },
    {
        filename: 'connect4',   
        label: 'Connect 4',
        description: 'A two-player game where you try to connect four of your colored discs in a row, column, or diagonal before your opponent does.',
        controls: [
            { key: 'Q', action: 'Move left' },
            { key: 'E', action: 'Move right' },
            { key: 'S', action: 'Place disc' },
        ],
        tips: ['Connect four discs in a row, column, or diagonal to win', 'Block your opponent from winning'],
        objective: 'Connect four discs in a row, column, or diagonal to win',
        speed: 2,
    },
    {
        filename: 'invaders',
        label: 'Invaders',
        description: 'The classic arcade game where you control a spaceship to shoot aliens before they land on your planet.',
        controls: [
            { key: 'Q', action: 'Move left' },
            { key: 'E', action: 'Move right' },
            { key: 'S', action: 'Place disc' },
        ],
        tips: ['Shoot all the aliens before they land'],
        objective: 'Destroy all the aliens to save the planet',
        speed: 12,
    },
    {
        filename: 'missile',
        label: 'Missile',
        description: 'Defend your cities from incoming missiles by shooting them down.',
        controls: [
            { key: 'S', action: 'Launch missile' },
        ],
        tips: ['Aim carefully to intercept incoming missiles', 'Protect your cities at all costs'],
        objective: 'Defend your cities from destruction',
        speed: 12,
    },
    {
        filename: 'pong',
        label: 'Pong',
        description: 'The classic table tennis game where you control a paddle to hit the ball back and forth.',
        controls: [
            { key: '1', action: 'Player 1 move paddle up' },
            { key: 'Q', action: 'Player 1 move paddle down' },
            { key: '4', action: 'Player 2 move paddle up' },
            { key: 'R', action: 'Player 2 move paddle down' },
        ],
        tips: ['Anticipate the ball\'s trajectory', 'Keep the ball in play to score points'],
        objective: 'Score more points than your opponent',
        speed: 12,
    },
    {
        filename: 'pong2',
        label: 'Pong 2',
        description: 'An advanced version of the classic Pong game with new features.',
        controls: [
            { key: '1', action: 'Player 1 move paddle up' },
            { key: 'Q', action: 'Player 1 move paddle down' },
            { key: '4', action: 'Player 2 move paddle up' },
            { key: 'R', action: 'Player 2 move paddle down' },
        ],
        tips: ['Anticipate the ball\'s trajectory', 'Keep the ball in play to score points'],
        objective: 'Win by scoring the most points',
        speed: 12,
    },
    {
        filename: 'snake',
        label: 'Snake',
        description: 'Guide the snake to eat food and grow longer without running into itself.',
        controls: [
            { key: '2', action: 'Turn Up' },
            { key: 'E', action: 'Turn Down' },
            { key: 'S', action: 'Turn Left' },
            { key: 'Q', action: 'Turn Right' },
        ],
        tips: ['Plan your moves ahead', 'Avoid crashing into the snake\'s body'],
        objective: 'Grow the longest snake possible',
        speed: 8,
    },
    {
        filename: 'tetris',
        label: 'Tetris',
        description: 'Fit falling blocks together to clear lines and score points.',
        controls: [
            { key: 'W', action: 'Move block left' },
            { key: 'E', action: 'Move block right' },
            { key: 'Q', action: 'Rotate block' },
            { key: 'A', action: 'Speed up descent' },
        ],
        tips: ['Clear lines to score points', 'Plan for upcoming blocks'],
        objective: 'Clear as many lines as possible',
        speed: 10,
    },
    {
        filename: 'wall',
        label: 'Wall',
        description: 'A game where you build walls to block your opponent.',
        controls: [
            { key: '1', action: 'Move up' },
            { key: 'Q', action: 'Move down' },
        ],
        tips: ['Anticipate the ball\'s trajectory', 'Keep the ball in play to score points'],
        objective: 'Outmaneuver your opponent by building walls',
        speed: 10,
    },
    {
        filename: 'wipeoff',
        label: 'Wipeoff',
        description: 'Clear the screen of obstacles as quickly as possible.',
        controls: [
            { key: 'Q', action: 'Move left' },
            { key: 'E', action: 'Move right' },
        ],
        tips: ['Use the paddle to bounce the ball off the top of the screen', 'Break all the bricks to win'],
        objective: 'Clear all obstacles from the screen',
        speed: 10,
    }
]