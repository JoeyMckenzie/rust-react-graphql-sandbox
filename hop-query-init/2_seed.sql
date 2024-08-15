INSERT INTO
    breweries (
        name,
        location,
        year_established,
        description,
        website
    )
VALUES (
        'Hops & Dreams Brewing Co.',
        'Portland, OR',
        2010,
        'Crafting hoppy dreams since 2010',
        'http://www.hopsanddreams.com'
    ),
    (
        'Malt Masters',
        'Denver, CO',
        2015,
        'Perfecting the art of malt',
        'http://www.maltmasters.com'
    ),
    (
        'Yeast Beast Brewery',
        'San Diego, CA',
        2018,
        'Unleashing the power of yeast',
        'http://www.yeastbeast.com'
    );

INSERT INTO
    beer_styles (name, description)
VALUES (
        'IPA',
        'India Pale Ale, a hoppy beer style'
    ),
    (
        'Stout',
        'Dark, roasty, and rich beer'
    ),
    (
        'Pilsner',
        'Crisp and clean lager'
    ),
    (
        'Wheat Beer',
        'Light and refreshing beer made with wheat'
    );

INSERT INTO
    beers (
        name,
        brewery_id,
        style_id,
        abv,
        ibu,
        description,
        is_seasonal
    )
VALUES (
        'Hoppy Wonder',
        1,
        1,
        6.5,
        65,
        'A wonderfully hoppy IPA',
        FALSE
    ),
    (
        'Midnight Velvet',
        2,
        2,
        5.5,
        30,
        'Smooth and silky stout',
        FALSE
    ),
    (
        'Crisp Breeze',
        3,
        3,
        4.8,
        20,
        'Refreshing pilsner for any occasion',
        FALSE
    ),
    (
        'Summer Wheat',
        1,
        4,
        4.2,
        15,
        'Light and fruity wheat beer',
        TRUE
    );

INSERT INTO
    ingredients (name, type, description)
VALUES (
        'Cascade',
        'hop',
        'Floral, citrus-like hop'
    ),
    (
        'Maris Otter',
        'malt',
        'Premium English pale malt'
    ),
    (
        'US-05',
        'yeast',
        'American Ale Yeast'
    ),
    (
        'Citra',
        'hop',
        'Strong citrus and tropical fruit hop'
    ),
    (
        'Roasted Barley',
        'malt',
        'Unmalted roasted barley for color and flavor'
    );

INSERT INTO
    beer_ingredients (
        beer_id,
        ingredient_id,
        amount
    )
VALUES (1, 1, '2 oz'),
    (1, 2, '10 lbs'),
    (1, 3, '1 packet'),
    (2, 5, '1 lb'),
    (3, 2, '8 lbs'),
    (4, 4, '1 oz');

-- Seed Reviews
INSERT INTO
    reviews (
        beer_id,
        user_name,
        rating,
        comment
    )
VALUES (
        1,
        'HopHead42',
        5,
        'Best IPA I''ve had in years!'
    ),
    (
        1,
        'BeerLover99',
        4,
        'Solid IPA, great hop profile'
    ),
    (
        2,
        'DarkBeerFan',
        5,
        'Smooth and rich, perfect stout'
    ),
    (
        3,
        'LagerLover',
        3,
        'Decent pilsner, but I''ve had better'
    ),
    (
        4,
        'SummerSipper',
        4,
        'Refreshing and perfect for hot days'
    );