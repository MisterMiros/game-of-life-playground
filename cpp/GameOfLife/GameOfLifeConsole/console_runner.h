#pragma once
#include <unordered_set>
#include <utility>
#include "../GameOfLifeEngine/cell.h"

class console_runner
{
    std::pair<unsigned int, unsigned int> read_grid_size();
    std::unordered_set<cell> read_initial_cells(unsigned int cols, unsigned int rows);
public:
    console_runner() = default;
    void run();
};
