#pragma once
#include <functional>
#include <vector>

class cell
{
    unsigned int x_;
    unsigned int y_;

public:
    cell() = default;
    cell(unsigned int x, unsigned int y);
    cell(const cell& other) = default;
    cell(cell&& other) noexcept = default;
    ~cell() = default;


    cell& operator=(const cell& other) = default;
    cell& operator=(cell&& other) noexcept = default;

    [[nodiscard]] unsigned int get_x() const;
    [[nodiscard]] unsigned int get_y() const;

    bool operator==(const cell& other) const;
    bool operator!=(const cell& other) const;
};

template <>
struct std::hash<cell>
{
    std::size_t operator()(const cell& c) const noexcept;
};
