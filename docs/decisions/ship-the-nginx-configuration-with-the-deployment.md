# Ship the nginx configuration with the deployment

- Status: Proposed
- Deciders: project owner
- Date: 2026-10-22
- Technical Story: The instance's front door is nginx. Its configuration lives in the repository and reaches the host with the deployment, so the server's routing is always the routing the repository states.

## Context and Problem Statement

The nginx configuration on the instance was copied there by hand. The deployment workflow shipped the server binary, the site folder and the allowlist file, and no configuration. Two things followed from that. The repository's `deploy/nginx/models.conf`, which carries the rewrite from the models subdomain to the models route, was reference material on the operator's machine and never reached the host, so the subdomain served the home page. And a routing mistake survived on the host long after it was corrected in the repository: the rule that sent every request under `/api/` to boltstore was written when boltstore took over the model data path, and it kept that path when the application arrived with its own calls under the same prefix. The site's own calls never reached the application, and every one of them answered with a routing error.

The second failure is the worse of the two. Sign-in, the models registry table, the settings page's data, the owner tools and the mailbox section all call the application over HTTP under `/api/`. They have answered with errors on the live site since the application was first deployed on 2026-10-09.

## Decision

The deployment carries the nginx configuration to the host.

The repository holds the front door's files under `deploy/nginx/`. The workflow copies each file into `/etc/nginx/conf.d/` on the instance, keeping a dated copy of the file it replaces, and then tests the whole configuration with `nginx -t`. A file that fails the test is put back the way it was, the test is run again, and the deployment stops there with the running configuration untouched. A configuration that passes the test is loaded with a reload, which keeps the connections that are open.

The routing follows from what each service answers:

- `/api/` goes to the application on `127.0.0.1:3000`. This is the path the application's own calls use, and the path the browser uses to reach them.
- `/store/` goes to boltstore on `127.0.0.1:8080`, with the prefix removed on the way, so boltstore's own paths are unchanged underneath it. The application reads boltstore from inside the instance, and the store's public path is for the operator.

The same two files serve both hosts: `exosystems.conf` for `exosystems.net` and `models.exosystems.net` is the one file that carries the models subdomain's rewrite, which the deployment now delivers with everything else.

## Consequences

The repository states the server's routing, and the deployment puts it there. A routing change is made in one place and reaches the host with the code that needs it.

A configuration that would not load cannot break the running site: the test runs before the reload, and the previous files are put back when the test fails. The dated copies of the files that were replaced stay in `/etc/nginx/conf.d/`, where the earlier hand-made copies already were.

The application's calls work on the live site, which is the condition every authenticated surface of the site depends on. The store's public path moves from `/api/` to `/store/`, and the store's dashboard was never built, so nothing on the host answered on that path.
