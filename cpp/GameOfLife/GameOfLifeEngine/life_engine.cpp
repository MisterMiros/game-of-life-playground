#include "life_engine.h"
#include <limits>
#include <stdexcept>

constexpr unsigned int max_grid = std::numeric_limits<unsigned int>::max() - 1;

life_engine::life_engine(const unsigned int cols, const unsigned int rows): cols_(cols), rows_(rows)
{
    alive_cells_ = std::unordered_set<cell>();
    potential_cells_ = std::unordered_set<cell>();
}

life_engine life_engine::create(const unsigned int cols, const unsigned int rows)
{
    if (cols < 1 || rows < 1 || cols > max_grid || rows > max_grid)
    {
        throw std::invalid_argument("Invalid grid size");
    }
    return {cols, rows};
}

void life_engine::activate_cell(unsigned int x, unsigned int y)
{
    alive_cells_.emplace(x, y);
    for (int i = -1; i <= 1; i++)
    {
        for (int j = -1; j <= 1; j++)
        {
            if (i == 0 && j == 0)
            {
                continue;
            }
            const unsigned int n_x = x + i;
            const unsigned int n_y = y + j;
            if (n_x <= cols_ - 1 && n_y < rows_ - 1)
            {
                potential_cells_.emplace(n_x, n_y);
            }
        }
    }
}

void life_engine::next()
{
    auto alive_cells_next = std::unordered_set<cell>(alive_cells_.size());
    auto potential_cells_next = std::unordered_set<cell>(potential_cells_.size());
    auto neighbours_buffer = std::vector<cell>(8);
    for (const auto& c : potential_cells_)
    {
        bool is_alive = alive_cells_.contains(c);
        this->get_neighbours(neighbours_buffer, c);
        int alive_neighbours = 0;
        for (auto neighbour : neighbours_buffer)
        {
            if (alive_cells_.contains(neighbour))
            {
                alive_neighbours++;
            }
        }

        if (is_alive)
        {
            if (alive_neighbours == 2 || alive_neighbours == 3)
            {
                alive_cells_next.emplace(c);
            }
            else
            {
                potential_cells_next.emplace(c);
                for (auto neighbour : neighbours_buffer)
                {
                    potential_cells_next.emplace(neighbour);
                }
            }
        } else
        {
            if (alive_neighbours == 3)
            {
                alive_cells_next.emplace(c);
                potential_cells_next.emplace(c);
                for (auto neighbour : neighbours_buffer)
                {
                    potential_cells_next.emplace(neighbour);
                }
            }
        }
    }

    this->alive_cells_ = std::move(alive_cells_next);
    this->potential_cells_ = std::move(potential_cells_next);
}

std::unordered_set<cell>::iterator life_engine::alive_cells_begin() const
{
    return alive_cells_.begin();
}

std::unordered_set<cell>::iterator life_engine::alive_cells_end() const
{
    return alive_cells_.end();
}

std::size_t life_engine::get_alive_cells_count() const
{
    return alive_cells_.size();
}

void life_engine::get_neighbours(std::vector<cell>& buffer, const cell& c) const
{
    buffer.clear();
    for (int i = -1; i <= 1; i++)
    {
        for (int j = -1; j <= 1; j++)
        {
            if (i == 0 && j == 0)
            {
                continue;
            }
            const auto x = c.get_x() + i;
            const auto y = c.get_y() + j;
            if (x <= cols_ - 1 && y < rows_ - 1)
            {
                buffer.emplace_back(x, y);
            }
        }
    }
}
