// See https://aka.ms/new-console-template for more information

using GameOfLifePixel;

var gameConfig = new GameConfig(1000, 1000);

var game = new Game(gameConfig);
game.Start();