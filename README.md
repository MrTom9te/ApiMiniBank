# 🏦 Mini Banco API - Requisitos Completos

## 📋 1. REQUISITOS FUNCIONAIS

### 1.1 Autenticação e Autorização
- **RF001**: Sistema deve permitir registro de novos usuários
  - Email único (validação)
  - Nome completo (min 2 palavras)
  - Senha (min 8 caracteres, 1 maiúscula, 1 número)
  - Hash da senha com bcrypt (cost 12)

- **RF002**: Sistema deve permitir login de usuários
  - Autenticação por email + senha
  - Geração de JWT com expiração de 24h
  - Refresh token opcional

- **RF003**: Sistema deve validar autenticação em rotas protegidas
  - Middleware de validação JWT
  - Extração do user_id do token
  - Retorno de erro 401 para tokens inválidos

### 1.2 Gestão de Usuários
- **RF004**: Sistema deve permitir consulta de perfil do usuário
- **RF005**: Sistema deve permitir atualização de dados do usuário
- **RF006**: Sistema deve permitir alteração de senha
- **RF007**: Sistema deve permitir desativação de conta

### 1.3 Gestão de Contas Bancárias
- **RF008**: Sistema deve permitir criação de contas bancárias
  - Usuário pode ter múltiplas contas
  - Tipos: corrente, poupança, investimento
  - Número da conta gerado automaticamente (10 dígitos)
  - Saldo inicial = 0.00

- **RF009**: Sistema deve listar contas do usuário logado
- **RF010**: Sistema deve consultar saldo de conta específica
- **RF011**: Sistema deve consultar detalhes da conta
- **RF012**: Sistema deve permitir desativação de conta

### 1.4 Transações Financeiras
- **RF013**: Sistema deve permitir depósitos
  - Valor > 0
  - Descrição obrigatória
  - Atualização do saldo
  - Registro da transação

- **RF014**: Sistema deve permitir saques
  - Validação de saldo suficiente
  - Valor > 0 e <= saldo disponível
  - Atualização do saldo
  - Registro da transação

- **RF015**: Sistema deve permitir transferências
  - Validação de conta origem (pertence ao usuário)
  - Validação de conta destino (existe)
  - Validação de saldo suficiente
  - Operação atômica (débito + crédito)
  - Registro de 2 transações (débito e crédito)

- **RF016**: Sistema deve consultar histórico de transações
  - Por conta específica
  - Ordenação por data (mais recente primeiro)
  - Paginação (50 registros por página)
  - Filtros: tipo, período, valor

### 1.5 Relatórios e Consultas
- **RF017**: Sistema deve gerar extrato por período
- **RF018**: Sistema deve calcular saldo disponível em tempo real
- **RF019**: Sistema deve consultar transações por tipo

## 🔧 2. REQUISITOS NÃO FUNCIONAIS

### 2.1 Performance
- **RNF001**: API deve responder em < 200ms para 95% das requisições
- **RNF002**: Suporte a pelo menos 1000 usuários simultâneos
- **RNF003**: Transações financeiras devem ser processadas em < 500ms

### 2.2 Segurança
- **RNF004**: Senhas devem ser hasheadas com bcrypt (cost >= 12)
- **RNF005**: JWT deve expirar em 24 horas
- **RNF006**: Rate limiting: 100 req/min por IP
- **RNF007**: HTTPS obrigatório em produção
- **RNF008**: Logs de auditoria para todas as transações
- **RNF009**: Validação de entrada em todos os endpoints

### 2.3 Disponibilidade
- **RNF010**: Uptime de 99.5%
- **RNF011**: Backup automático do banco (diário)
- **RNF012**: Recovery em caso de falha < 5 minutos

### 2.4 Escalabilidade
- **RNF013**: Arquitetura preparada para load balancing
- **RNF014**: Banco de dados com índices otimizados
- **RNF015**: Cache para consultas frequentes

## 🗄️ 3. MODELO DE DADOS

### 3.1 Tabela: users
```sql
- id: UUID (PK)
- email: VARCHAR(255) UNIQUE NOT NULL
- name: VARCHAR(255) NOT NULL
- password_hash: VARCHAR(255) NOT NULL
- is_active: BOOLEAN DEFAULT true
- created_at: TIMESTAMP
- updated_at: TIMESTAMP
```

### 3.2 Tabela: accounts
```sql
- id: UUID (PK)
- user_id: UUID (FK -> users.id)
- account_number: VARCHAR(20) UNIQUE NOT NULL
- account_type: ENUM('checking', 'savings', 'investment')
- balance: DECIMAL(15,2) DEFAULT 0.00
- is_active: BOOLEAN DEFAULT true
- created_at: TIMESTAMP
- updated_at: TIMESTAMP
```

### 3.3 Tabela: transactions
```sql
- id: UUID (PK)
- from_account_id: UUID (FK -> accounts.id) NULL
- to_account_id: UUID (FK -> accounts.id) NULL
- amount: DECIMAL(15,2) NOT NULL
- transaction_type: ENUM('deposit', 'withdraw', 'transfer_debit', 'transfer_credit')
- description: TEXT NOT NULL
- reference_id: UUID NULL (para linking transferências)
- status: ENUM('pending', 'completed', 'failed') DEFAULT 'completed'
- created_at: TIMESTAMP
```

### 3.4 Índices Necessários
```sql
-- Users
CREATE INDEX idx_users_email ON users(email);

-- Accounts
CREATE INDEX idx_accounts_user_id ON accounts(user_id);
CREATE INDEX idx_accounts_number ON accounts(account_number);

-- Transactions
CREATE INDEX idx_transactions_from_account ON transactions(from_account_id);
CREATE INDEX idx_transactions_to_account ON transactions(to_account_id);
CREATE INDEX idx_transactions_created_at ON transactions(created_at);
CREATE INDEX idx_transactions_type ON transactions(transaction_type);
```

## 🔗 4. ENDPOINTS DA API

### 4.1 Autenticação
```
POST   /api/v1/auth/register     - Registro de usuário
POST   /api/v1/auth/login        - Login
POST   /api/v1/auth/refresh      - Refresh token
POST   /api/v1/auth/logout       - Logout
```

### 4.2 Usuários
```
GET    /api/v1/users/profile     - Perfil do usuário
PUT    /api/v1/users/profile     - Atualizar perfil
PUT    /api/v1/users/password    - Alterar senha
DELETE /api/v1/users/account     - Desativar conta
```

### 4.3 Contas
```
POST   /api/v1/accounts          - Criar conta
GET    /api/v1/accounts          - Listar contas
GET    /api/v1/accounts/:id      - Detalhes da conta
PUT    /api/v1/accounts/:id      - Atualizar conta
DELETE /api/v1/accounts/:id      - Desativar conta
GET    /api/v1/accounts/:id/balance - Consultar saldo
```

### 4.4 Transações
```
POST   /api/v1/accounts/:id/deposit    - Depósito
POST   /api/v1/accounts/:id/withdraw   - Saque
POST   /api/v1/accounts/:id/transfer   - Transferência
GET    /api/v1/accounts/:id/transactions - Histórico
GET    /api/v1/accounts/:id/statement  - Extrato por período
```

## 📝 5. ESTRUTURAS DE REQUEST/RESPONSE

### 5.1 Request Bodies
```typescript
// Register
{
  email: string;
  name: string;
  password: string;
}

// Login
{
  email: string;
  password: string;
}

// Create Account
{
  account_type: 'checking' | 'savings' | 'investment';
}

// Deposit/Withdraw
{
  amount: number;
  description: string;
}

// Transfer
{
  to_account_number: string;
  amount: number;
  description: string;
}
```

### 5.2 Response Format
```typescript
// Padrão de resposta
{
  success: boolean;
  data?: any;
  message: string;
  error?: string;
  timestamp: string;
}

// Paginação
{
  success: boolean;
  data: any[];
  pagination: {
    page: number;
    limit: number;
    total: number;
    pages: number;
  };
  message: string;
}
```

## ⚠️ 6. VALIDAÇÕES E REGRAS DE NEGÓCIO

### 6.1 Usuários
- Email deve ser válido e único
- Nome deve ter pelo menos 2 palavras
- Senha: min 8 chars, 1 maiúscula, 1 minúscula, 1 número

### 6.2 Contas
- Usuário pode ter múltiplas contas
- Número da conta deve ser único globalmente
- Saldo não pode ser negativo
- Apenas o dono pode operar a conta

### 6.3 Transações
- Valores devem ser > 0
- Saque: valor <= saldo disponível
- Transferência: conta origem deve pertencer ao usuário
- Transferência: conta destino deve existir e estar ativa
- Descrição é obrigatória
- Transações são irreversíveis

### 6.4 Rate Limiting
- Login: 5 tentativas por 15 minutos por IP
- Transações: 10 por minuto por usuário
- Consultas: 100 por minuto por usuário

## 🛠️ 7. STACK TÉCNICA SUGERIDA

### 7.1 Backend (Rust)
```toml
[dependencies]
actix-web = "4.4"           # Framework web
sqlx = "0.7"                # Database toolkit
tokio = "1.35"              # Runtime assíncrono
serde = "1.0"               # Serialization
uuid = "1.6"                # UUIDs
chrono = "0.4"              # Date/time
bcrypt = "0.15"             # Password hashing
jsonwebtoken = "9.2"        # JWT
rust_decimal = "1.33"       # Decimal precision
validator = "0.16"          # Input validation
tracing = "0.1"             # Logging
redis = "0.23"              # Cache (opcional)
```

### 7.2 Banco de Dados
- **PostgreSQL 15+** (produção)
- **SQLite** (desenvolvimento)
- **Redis** (cache e sessions)

### 7.3 Infraestrutura
- **Docker** para containerização
- **Docker Compose** para ambiente local
- **Nginx** como reverse proxy
- **Let's Encrypt** para SSL

## 🧪 8. TESTES NECESSÁRIOS

### 8.1 Testes Unitários
- Validação de dados
- Hash de senhas
- Geração de JWT
- Cálculos de saldo
- Regras de negócio

### 8.2 Testes de Integração
- Fluxo completo de registro → login → transação
- Operações no banco de dados
- Middleware de autenticação
- APIs endpoints

### 8.3 Testes de Performance
- Carga de 1000 usuários simultâneos
- Stress test em transações
- Tempo de resposta dos endpoints

## 📊 9. MONITORAMENTO E LOGS

### 9.1 Logs Necessários
- Todas as transações financeiras
- Tentativas de login (sucesso/falha)
- Erros de sistema
- Performance de queries
- Rate limiting hits

### 9.2 Métricas
- Requests per second
- Response times
- Error rates
- Active users
- Transaction volume
- Database performance

## 🚀 10. DEPLOYMENT

### 10.1 Ambiente de Desenvolvimento
```bash
# Docker Compose
- PostgreSQL
- Redis
- API server
- Hot reload
```

### 10.2 Ambiente de Produção
```bash
# Kubernetes ou Docker Swarm
- Load balancer (Nginx)
- API replicas (3+)
- PostgreSQL cluster
- Redis cluster
- Backup automation
- SSL termination
```

## 📚 11. DOCUMENTAÇÃO

### 11.1 API Documentation
- OpenAPI/Swagger spec
- Postman collection
- Request/Response examples
- Error codes reference

### 11.2 Developer Documentation
- Setup instructions
- Architecture overview
- Database schema
- Deployment guide
- Contributing guidelines

---

## 🏁 ROADMAP DE IMPLEMENTAÇÃO

### Fase 1: Base (1-2 semanas)
1. Setup do projeto Rust + Actix Web
2. Configuração do banco PostgreSQL
3. Autenticação (registro + login)
4. Middleware JWT
5. Testes básicos

### Fase 2: Core Features (2-3 semanas)
1. CRUD de contas
2. Transações básicas (depósito, saque)
3. Transferências entre contas
4. Histórico de transações
5. Validações de negócio

### Fase 3: Refinamento (1-2 semanas)
1. Rate limiting
2. Logs estruturados
3. Tratamento de erros
4. Documentação da API
5. Testes de integração

### Fase 4: Produção (1 semana)
1. Docker + Docker Compose
2. Configuração de produção
3. SSL/HTTPS
4. Monitoramento básico
5. Backup do banco
