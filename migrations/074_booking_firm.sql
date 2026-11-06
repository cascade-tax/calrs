-- Cascade: the optional firm a guest enters on the booking form. When set,
-- the calendar invitation is titled "Cascade <> {firm}" instead of the
-- event type name.
ALTER TABLE bookings ADD COLUMN firm TEXT;
