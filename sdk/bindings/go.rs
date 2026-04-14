pub struct GoBinding;

impl GoBinding {
    pub fn generate() -> String {
        r#"
package vault

import "net/http"
import "bytes"

type VaultClient struct {
    Endpoint string
}

func (v *VaultClient) Store(data string) (*http.Response, error) {
    return http.Post(v.Endpoint+"/store", "text/plain", bytes.NewBuffer([]byte(data)))
}

func (v *VaultClient) Get() (*http.Response, error) {
    return http.Get(v.Endpoint + "/get")
}
"#
        .to_string()
    }
}
