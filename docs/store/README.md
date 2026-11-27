# Microsoft Store Package (MSIX) for CrawRAD

CrawRAD provides an MSIX package for distribution through the Microsoft Store.
The Microsoft Store signs the MSIX package itself during publication, so no private code signing certificate is required.

## Repository Variables

Building the MSIX package requires three repository variables configured in GitHub (or passed via environment variables locally):

- `STORE_IDENTITY_NAME`: The Package Identity Name
- `STORE_PUBLISHER`: The Package Publisher ID (e.g. `CN=...`)
- `STORE_PUBLISHER_DISPLAY`: The Publisher Display Name

### Where to Find Them in Partner Center

In [Microsoft Partner Center](https://partner.microsoft.com/dashboard):
1. Navigate to your app overview.
2. Go to **Product management** > **Product identity**.
3. Copy the fields:
   - **Package/Identity/Name** → `STORE_IDENTITY_NAME`
   - **Package/Identity/Publisher** → `STORE_PUBLISHER`
   - **Package/Properties/PublisherDisplayName** → `STORE_PUBLISHER_DISPLAY`

If any of these three variables is missing during the release build, the MSIX packaging step will output a notification and safely skip execution without failing the pipeline.

## Building Locally

1. Build the install bundle first:
   ```bash
   python tools/bundle/bundle.py
   ```

2. Set the three environment variables:
   ```powershell
   $env:STORE_IDENTITY_NAME = "YourStoreIdentityName"
   $env:STORE_PUBLISHER = "CN=YourPublisherId"
   $env:STORE_PUBLISHER_DISPLAY = "Your Publisher Display Name"
   ```

3. Run the packager script:
   ```powershell
   python tools/bundle/msix.py --bundle target\bundle\CrawRAD --version 0.1.0 --out target\installer
   ```

The resulting package `CrawRAD-0.1.0-windows-x64.msix` will be written to `target\installer\`.

## Signing

MSIX packages submitted to the Microsoft Store do not require a self-signed or purchased certificate. Microsoft Store applies its own trusted signature to the package during ingestion.
