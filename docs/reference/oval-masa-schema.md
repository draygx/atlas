# Oval Masa — İmece Entegrasyon Şeması

> Bu belge, İmece'nin gelecekte Oval Masa (GTK3+VTE çok-CLI yönetim merkezi)
> ile entegrasyonu için hazırlanmış bir tasarım kapısıdır. Henüz uygulanmamıştır.

## Amaç

Oval Masa, Claude Code / Agy / Qwen CLI oturumlarını tek pencerede yönetir.
İmece checkpoint'lerini bu oturumlarla ilişkilendirerek hangi agent
oturumunun hangi git commit'ine karşılık geldiğini gösterir.

## Entegrasyon Yöntemi

İmece bir SQLite veritabanı dosyasına yazar; Oval Masa bu dosyayı salt okunur olarak izler.
Doğrudan IPC yerine dosya tabanlı köprü seçildi — her iki uygulama bağımsız çalışır ve
birinin çökmesi diğerini etkilemez.

**Konum:** `~/.local/share/imece/oval_masa_bridge.db`

## Tablo Şeması

```sql
-- Bir agent oturumunun başladığını ve sonlandığını kaydeder.
CREATE TABLE IF NOT EXISTS agent_sessions (
    id          TEXT PRIMARY KEY,          -- UUID (İmece'nin kendi session ID'si)
    agent_name  TEXT NOT NULL,             -- "claude-code" | "codex" | "agy" | custom
    project_dir TEXT NOT NULL,             -- Çalışma dizini (tam yol)
    started_at  INTEGER NOT NULL,          -- Unix epoch (milisaniye)
    ended_at    INTEGER,                   -- NULL = hâlâ aktif
    git_branch  TEXT,                      -- Başlangıçtaki branch adı
    git_head    TEXT                       -- Başlangıçtaki HEAD commit SHA
);

-- İmece'nin kaydettiği her checkpoint.
CREATE TABLE IF NOT EXISTS checkpoints (
    id          TEXT PRIMARY KEY,          -- UUID
    session_id  TEXT NOT NULL REFERENCES agent_sessions(id),
    commit_sha  TEXT NOT NULL,             -- Git commit SHA
    commit_msg  TEXT,                      -- İlk satır
    created_at  INTEGER NOT NULL,          -- Unix epoch (milisaniye)
    agent_turn  INTEGER,                   -- Kaçıncı agent turu tetikledi (opsiyonel)
    note        TEXT                       -- Kullanıcı notu (opsiyonel)
);

-- Oval Masa tarafından yazılır: hangi VTE sekmesi hangi oturum ile eşleşiyor.
CREATE TABLE IF NOT EXISTS oval_masa_tabs (
    tab_id      TEXT PRIMARY KEY,          -- Oval Masa sekme UUID'si
    session_id  TEXT REFERENCES agent_sessions(id),
    tab_title   TEXT,
    linked_at   INTEGER                    -- Unix epoch (milisaniye)
);
```

## Kullanım Akışı

1. İmece bir agent oturumu başlattığında `agent_sessions`'a kayıt ekler.
2. Kullanıcı checkpoint aldığında `checkpoints`'e commit SHA yazar.
3. Oval Masa veritabanını 5 saniyede bir okur (`PRAGMA journal_mode=WAL` ile güvenli).
4. Oval Masa sekme başlığında `[checkpoint ✓]` gösterebilir.

## Şu An Gerekli Değil

Bu şema henüz uygulanmamıştır. İmece veya Oval Masa bu tabloları yazmaz.
Entegrasyona başlamadan önce:
- [ ] İmece tarafında Tauri command: `create_checkpoint_bridge_record`
- [ ] Oval Masa tarafında: `~/.local/share/imece/oval_masa_bridge.db` izleyici
