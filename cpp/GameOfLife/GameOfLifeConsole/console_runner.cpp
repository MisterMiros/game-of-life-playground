#include "console_runner.h"
#include <chrono>
#include <iostream>
#include <string>
#include "../GameOfLifeEngine/life_engine.h"

std::vector<std::string> split(std::string s, std::string delimiter)
{
    size_t pos_start = 0, pos_end, delim_len = delimiter.length();
    std::string token;
    std::vector<std::string> res;

    while ((pos_end = s.find(delimiter, pos_start)) != std::string::npos)
    {
        token = s.substr(pos_start, pos_end - pos_start);
        pos_start = pos_end + delim_len;
        res.push_back(token);
    }

    res.push_back(s.substr(pos_start));
    return res;
}

const std::string invalid_grid_format_message = "Invalid grid format, aborting";

std::pair<unsigned int, unsigned int> console_runner::read_grid_size()
{
    std::string input;
    std::cin >> input;
    if (input.empty())
    {
        throw std::invalid_argument(invalid_grid_format_message);
    }
    std::vector<std::string> tokens = split(input, ",");
    if (tokens.size() != 2)
    {
        throw std::invalid_argument(invalid_grid_format_message);
    }
    return {std::stoul(tokens[0]), std::stoul(tokens[1])};
}

const std::string invalid_cell_format_message = "Invalid cell format, aborting";

std::unordered_set<cell> console_runner::read_initial_cells(const unsigned int cols, const unsigned int rows)
{
    std::unordered_set<cell> initial_cells = {};
    while (true)
    {
        std::string input;
        std::cin >> input;
        if (input == "END")
        {
            break;
        }

        std::vector<std::string> tokens = split(input, ",");
        if (tokens.size() != 2)
        {
            throw std::invalid_argument(invalid_cell_format_message);
        }
        const unsigned int x = std::stoul(tokens[0]);
        const unsigned int y = std::stoul(tokens[1]);
        if (x >= cols || y >= rows)
        {
            throw std::invalid_argument(
                "Invalid cell position: (" + std::to_string(x) + ", " + std::to_string(y) + "), aborting");
        }
        initial_cells.emplace(std::stoul(tokens[0]), std::stoul(tokens[1]));
    }
    return initial_cells;
}

void console_runner::run()
{
    std::cout << "Running Game of Life in console..." << '\n';
    std::cout << "Enter the size of the grid (columns and rows) using the following format: cols,rows" << '\n';
    std::pair<unsigned int, unsigned int> grid_size = this->read_grid_size();

    std::cout << "Enter the initial cell configuration using the following format:\n"
        << "- Each line should contain one cell position as x,y coordinates\n"
        << "- Type 'END' on a new line when you have finished entering all cells\n";

    std::unordered_set<cell> initial_cells = this->read_initial_cells(grid_size.first, grid_size.second);

    life_engine engine = life_engine::create(grid_size.first, grid_size.second);
    for (const auto& c : initial_cells)
    {
        engine.activate_cell(c.get_x(), c.get_y());
    }

    std::cout << "Initial alive cells: " << initial_cells.size() << '\n';
    std::cout << "Press 'N' to run the next generation, 'Q' to quit" << '\n';

    while (true)
    {
        std::string input;
        std::cin >> input;
        if (input == "N")
        {
            auto start = std::chrono::steady_clock::now();
            engine.next();
            auto end = std::chrono::steady_clock::now();
            auto elapsed = std::chrono::duration_cast<std::chrono::milliseconds>(end - start);
            std::cout << "Next generation is ready. Active cells: " << engine.get_alive_cells_count() <<
                ". Elapsed time: " << elapsed.count() << " ms" << '\n';
        }
        else
        {
            break;
        }
    }

    std::cout << "Game of Life finished" << '\n';
}
