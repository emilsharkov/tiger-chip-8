export interface Rom {
    filename: string;
    label: string;  
}

export const ROMS: Rom[] = [
    {
        filename: 'breakout',
        label: 'Breakout'
    },
    {
        filename: 'connect4',   
        label: 'Connect 4'
    },
    {
        filename: 'invaders',
        label: 'Invaders'
    },
    {
        filename: 'missile',
        label: 'Missile'
    },
    {
        filename: 'pong',
        label: 'Pong'
    },
    {
        filename: 'pong2',
        label: 'Pong 2'
    },
    {
        filename: 'snake',
        label: 'Snake'
    },
    {
        filename: 'tetris',
        label: 'Tetris'
    },
    {
        filename: 'wall',
        label: 'Wall'
    },
    {
        filename: 'wipeoff',
        label: 'Wipeoff'
    }
]