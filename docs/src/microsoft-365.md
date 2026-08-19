# Microsoft 365 Calendar

calrs connects to Office 365 calendars through Microsoft Graph using delegated OAuth2 access. Each person authorizes their own work or school account. Access and refresh tokens are encrypted at rest, and confirmed bookings can be written back to a selected Microsoft calendar.

The older **Microsoft Exchange (EWS)** source remains available for on-prem Exchange. Microsoft 365 should use this Graph integration.

## Register the Entra application

1. Open **Microsoft Entra admin center → App registrations → New registration**.
2. Choose the account scope for your deployment. A single-tenant registration is simplest for one organization; a multi-tenant registration works with the `organizations` tenant setting in calrs.
3. Under **Authentication**, add a **Web** redirect URI:

   ```text
   https://your-cascade-host/dashboard/sources/microsoft/callback
   ```

   If the CLI connection command will be used, add this second **Web** redirect URI as well:

   ```text
   http://localhost:8400/callback
   ```

4. Under **API permissions**, add these delegated Microsoft Graph permissions:

   - `User.Read` — identifies the Microsoft account that authorized the connection.
   - `Calendars.ReadWrite` — lists calendars, reads busy events, and creates or deletes Cascade booking events.

5. Under **Certificates & secrets**, create a client secret and copy its value before leaving the page.

calrs requests `offline_access`, which lets it refresh access without asking the user to sign in for every sync.

## Configure calrs

The public base URL must be configured first. In **Admin → Microsoft 365 Calendar**, enter:

- **Tenant** — `organizations` for any Microsoft work or school account, or one Entra tenant ID/domain.
- **Application (client) ID** — from the Entra app registration.
- **Client secret** — the secret value, not its secret ID. Leaving it empty on a later save keeps the stored value.

The client secret is encrypted with the same AES-256-GCM key used for other stored credentials.

## Connect a calendar

From **Calendar sources → Add source**, choose the **Microsoft 365 (Graph)** backend and the **Microsoft 365 / Office 365** preset, then select **Connect with Microsoft**. After consent, calrs discovers the calendars available to that account, syncs their events, and asks which calendar should receive confirmed bookings.

The CLI flow is:

```bash
calrs source add-microsoft
calrs sync
```

The CLI starts a temporary localhost callback and opens the Microsoft authorization page.

## Sync and write-back behavior

- Sync reads a bounded calendar view from 90 days in the past through roughly two years in the future. Microsoft Graph expands recurring events and exceptions inside that window.
- Events marked free remain non-blocking; busy events block booking slots.
- Booking write-back creates an appointment without Microsoft-generated attendee invitations because Cascade sends its own booking email. A private extended property ties the Graph event to the Cascade booking UID for reliable update and cancellation.
- Access tokens refresh automatically. If consent is revoked, the source test and sync surfaces report the authorization failure and the user can remove and reconnect the source.

Microsoft Graph endpoints and permissions are documented in Microsoft's [delegated access guide](https://learn.microsoft.com/graph/auth-v2-user), [calendar overview](https://learn.microsoft.com/graph/api/resources/calendar-overview), and [calendar view API](https://learn.microsoft.com/graph/api/user-list-calendarview).
