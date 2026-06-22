# GeoConnect

A mobile app for American-Japanese cultural exchange. Users are able to click on Japan's 8 regions, read cultural and historical information about each one, and compare US vs. Japanese customs with each other. The app is being built with Dioxus (Rust) for the UI and a PostgreSQL database (hosted on Railway).

## Repository

https://github.com/logerator/GeoConnect

## Requirements

- cargo
- A .env file with a DATABASE_URL pointing to a database hosted on Railway (link included in submission)

## Build & Install

cargo install dioxus-cli --locked
cargo build

## How to Run

dx serve

## Database Schema

regions - the 8 regions of Japan (id, name, overview)
categories - content topics like Food, Etiquette, Festivals, History (id, name)
facts - cultural/historical content per region and category (id, region_id, category_id, title, body)
comparisons - US vs. Japanese custom comparisons, optionally tied to a region (id, category_id, region_id, title, us_practice, jp_practice)