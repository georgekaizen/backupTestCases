<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description>Agent Product Addittion sms Template</description>
   <name>Agent product Addition sms template with invalid template id</name>
   <tag></tag>
   <elementGuidId>5256cca5-7127-4e82-9550-86c463afce84</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;contentType&quot;: &quot;application/x-www-form-urlencoded&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;,
  &quot;parameters&quot;: [
    {
      &quot;name&quot;: &quot;recipients&quot;,
      &quot;value&quot;: &quot;254716684956&quot;
    },
    {
      &quot;name&quot;: &quot;template_id&quot;,
      &quot;value&quot;: &quot;9&quot;
    },
    {
      &quot;name&quot;: &quot;first_name&quot;,
      &quot;value&quot;: &quot;magunas&quot;
    },
    {
      &quot;name&quot;: &quot;product_name&quot;,
      &quot;value&quot;: &quot;chrome book&quot;
    },
    {
      &quot;name&quot;: &quot;outlet&quot;,
      &quot;value&quot;: &quot;Friesen Group&quot;
    },
    {
      &quot;name&quot;: &quot;product_code&quot;,
      &quot;value&quot;: &quot;Tusk_12&quot;
    },
    {
      &quot;name&quot;: &quot;phone_no&quot;,
      &quot;value&quot;: &quot;0719725060&quot;
    }
  ]
}</httpBodyContent>
   <httpBodyType>x-www-form-urlencoded</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/x-www-form-urlencoded</value>
   </httpHeaderProperties>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>http://176.58.103.227:8086/api/send_sms</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceFunction></soapServiceFunction>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()
</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
