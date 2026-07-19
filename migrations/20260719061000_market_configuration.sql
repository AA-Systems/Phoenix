ALTER TABLE assets
    ALTER COLUMN symbol SET NOT NULL,
    ADD CONSTRAINT assets_decimals_range CHECK (decimals BETWEEN 0 AND 18);

ALTER TABLE markets
    ALTER COLUMN symbol SET NOT NULL,
    ADD COLUMN price_tick_size BIGINT NOT NULL DEFAULT 1,
    ADD COLUMN quantity_step_size BIGINT NOT NULL DEFAULT 1,
    ADD COLUMN min_order_quantity BIGINT NOT NULL DEFAULT 1,
    ADD COLUMN min_order_notional BIGINT NOT NULL DEFAULT 1,
    ADD CONSTRAINT markets_price_tick_size_positive CHECK (price_tick_size > 0),
    ADD CONSTRAINT markets_quantity_step_size_positive CHECK (quantity_step_size > 0),
    ADD CONSTRAINT markets_min_order_quantity_positive CHECK (min_order_quantity > 0),
    ADD CONSTRAINT markets_min_order_notional_positive CHECK (min_order_notional > 0);

ALTER TABLE markets
    ALTER COLUMN price_tick_size DROP DEFAULT,
    ALTER COLUMN quantity_step_size DROP DEFAULT,
    ALTER COLUMN min_order_quantity DROP DEFAULT,
    ALTER COLUMN min_order_notional DROP DEFAULT;
