#include "cell.h"

cell::cell(const unsigned int x, const unsigned int y): x_(x), y_(y)
{
}

unsigned int cell::get_x() const
{
    return this->x_;
}

unsigned int cell::get_y() const
{
    return this->y_;
}

bool cell::operator==(const cell& other) const
{
    return this->x_ == other.x_ && this->y_ == other.y_;
}

bool cell::operator!=(const cell& other) const
{
    return this->x_ != other.x_ || this->y_ != other.y_;
}

std::size_t std::hash<cell>::operator()(const cell& c) const noexcept
{
    // Combine x and y coordinates into a single hash
    const std::size_t h1 = std::hash<unsigned int>{}(c.get_x());
    const std::size_t h2 = std::hash<unsigned int>{}(c.get_y());

    // Common way to combine two hash values
    return h1 ^ (h2 << 1);
}
