#pragma once

#include "cell.h"
#include <unordered_set>

class life_engine
{
    life_engine(unsigned int cols, unsigned int rows);

    const unsigned int cols_;
    const unsigned int rows_;
    std::unordered_set<cell> alive_cells_;
    std::unordered_set<cell> potential_cells_;

    void get_neighbours(std::vector<cell>& buffer, const cell& c) const;

public:
    static life_engine create(unsigned int cols, unsigned int rows);

    void activate_cell(unsigned int x, unsigned int y);
    void next();
    std::unordered_set<cell>::iterator alive_cells_begin() const;
    std::unordered_set<cell>::iterator alive_cells_end() const;
    std::size_t get_alive_cells_count() const;
};
