# SPDX-License-Identifier: LicenseRef-Ferrocene
# SPDX-FileCopyrightText: Ferrous Systems and AdaCore

from . import substitutions, document_id, domain, signature_page
import string


def setup(app):
    substitutions.setup(app)
    document_id.setup(app)
    domain.setup(app)
    signature_page.setup(app)

    app.connect("config-inited", validate_config)
    app.add_config_value("ferrocene_id", None, "env", [str])
    app.add_config_value("ferrocene_substitutions_path", None, "env", [str])
    app.add_config_value("ferrocene_signed", False, "env", [str])

    return {
        "version": "0",
        "parallel_read_safe": True,
        "parallel_write_safe": True,
    }


def validate_config(app, config):
    for required in ["ferrocene_id", "ferrocene_substitutions_path"]:
        if config[required] is None:
            raise ValueError(f"Missing required {required} configuration")

    if any(c not in string.ascii_uppercase for c in config["ferrocene_id"]):
        raise ValueError("ferrocene_id can only be uppercase letters")
