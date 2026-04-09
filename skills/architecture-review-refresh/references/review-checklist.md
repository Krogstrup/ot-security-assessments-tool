<!-- markdownlint-disable MD013 MD029 MD032 -->

# Review Checklist

Use this checklist while refreshing:

- `ARCHITECTURE_REVIEW.md`
  - runtime entrypoint and route modules confirmed
  - application extractions listed from `backend/src/application/*`
  - command-heavy hotspots quantified
  - dependency leakage and error-contract status stated
  - incremental backend roadmap included

- `SYSTEM_ARCHITECTURE_REVIEW.md`
  - frontend-backend runtime topology updated
  - cross-layer contract status (errors/validation) updated
  - top cross-system risks listed
  - phased roadmap and issue set included

- `FRONTEND_ARCHITECTURE_REVIEW.md`
  - root shell orchestration ownership updated (`+page.svelte`)
  - extracted flow modules vs view-centric modules listed
  - API validation coverage listed by module
  - incremental frontend roadmap included

## Command Patterns

Run targeted discovery commands before writing:

```bash
# Backend runtime + routes
sed -n '1,220p' backend/Cargo.toml
sed -n '1,220p' backend/src/bin/kusanaginokajiki_web.rs
sed -n '1,260p' backend/src/bin/kusanaginokajiki_web/web_routes.rs
sed -n '1,320p' backend/src/bin/kusanaginokajiki_web/web_api_paths.rs

# Backend extraction map
find backend/src/application -maxdepth 3 -type f | sort
sed -n '1,220p' backend/src/application/mod.rs
sed -n '1,280p' backend/src/commands/mod.rs

# Frontend structure + hotspots
sed -n '1,260p' src/routes/+page.svelte
find src/lib/components -maxdepth 2 -type d | sort
find src/lib/stores -maxdepth 2 -type f | sort
find src/lib/api -maxdepth 2 -type f | sort
wc -l backend/src/commands/*.rs src/routes/+page.svelte src/lib/components/*View.svelte | sort -nr

# Validation usage
rg -n "httpValidated|httpJson" src/lib/api
```

## Final Validation

```bash
npx -y markdownlint-cli2@latest ARCHITECTURE_REVIEW.md SYSTEM_ARCHITECTURE_REVIEW.md FRONTEND_ARCHITECTURE_REVIEW.md skills/architecture-review-refresh/SKILL.md skills/architecture-review-refresh/references/review-checklist.md
python3 /home/admin/.codex/skills/.system/skill-creator/scripts/quick_validate.py skills/architecture-review-refresh
```
