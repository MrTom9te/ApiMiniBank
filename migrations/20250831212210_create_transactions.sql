-- ========================
-- Tabela: transactions
-- ========================
CREATE TABLE IF NOT EXISTS transactions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    from_account_id UUID NULL REFERENCES accounts(id) ON DELETE SET NULL,
    to_account_id UUID NULL REFERENCES accounts(id) ON DELETE SET NULL,
    amount DECIMAL(15,2) NOT NULL,
    transaction_type transaction_type_enum NOT NULL,
    description TEXT NOT NULL,
    reference_id UUID NULL,
    status transaction_status_enum DEFAULT 'pending',
    created_at TIMESTAMP DEFAULT now()
);
