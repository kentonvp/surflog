import logging
import os
import sqlite3
from pathlib import Path
from typing import Optional

logger = logging.getLogger(__name__)
logging.basicConfig(
    level=logging.WARNING, format="%(asctime)s - %(levelname)s - %(message)s"
)

DATABASE = Path(os.getenv("DATABASE", "~/.config/surflog.db")).expanduser()
MIGRATION_DIR = Path(__file__).parent


def get_current_migration() -> Optional[str]:
    """Return the current migration applied to the database, or None if no migrations have been applied."""
    try:
        conn = sqlite3.connect(DATABASE)
        try:
            cursor = conn.cursor()
            cursor.execute("SELECT * FROM current_migrations LIMIT 1;")
            resp = cursor.fetchone()  # returns a unnamed tuple
        except Exception:
            raise
        finally:
            conn.close()

        if resp:
            return resp[0]
    except Exception as e:
        logger.warning(f"Uh oh: {e}")
        pass
    return None


def apply_migration(migration_file: Path) -> None:
    """Apply a migration from the given SQL file to the database."""
    conn = sqlite3.connect(DATABASE)
    try:
        with migration_file.open("r") as f:
            sql_script = f.read()
        cursor = conn.cursor()
        cursor.executescript(sql_script)
        conn.commit()
        logger.info(f"Applied {migration_file.name!r} successfully")
    except Exception:
        logger.exception(f"Failed to apply migration {migration_file.name!r}")
        raise
    finally:
        conn.close()


def by_stem(path: Path) -> str:
    return path.stem


if __name__ == "__main__":
    try:
        logger.debug(f"Migration directory: {MIGRATION_DIR!r}")
        logger.debug(f"Database path: {DATABASE!r}")

        current_migration = get_current_migration()
        logger.debug(f"Current migration: {current_migration!r}")
        migrations = sorted((p for p in MIGRATION_DIR.glob("v*.sql")), key=by_stem)
        logger.debug(f"Found {len(migrations)} migrations")
        applied_cnt = 0
        while migrations:
            mig = migrations.pop(0)
            logger.debug(f"Migration? {mig!r}")
            if current_migration is not None and current_migration >= mig.stem:
                logger.debug(f"Older migration already applied: {mig!r}")
                continue

            apply_migration(MIGRATION_DIR / mig)
            current_migration = mig
            applied_cnt += 1

        if applied_cnt:
            print(f"💊 Applied {applied_cnt} migrations - migrain managed.")
        else:
            print("🤨 No migrations - you have a migrain?")

    except Exception:
        logger.exception("An error occurred during migration check.")
