// See https://aka.ms/new-console-template for more information

using GameOfLifePixel;

var gameConfig = new GameConfig(10000, 10000);

var game = new Game(gameConfig);
game.Start();