<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description>goal booking sms template</description>
   <name>Goal Booking Sms Template with invalid date format inputted</name>
   <tag></tag>
   <elementGuidId>6aed88f0-fedd-4c5b-a08c-22f9e44975a1</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;contentType&quot;: &quot;application/x-www-form-urlencoded&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;,
  &quot;parameters&quot;: [
    {
      &quot;name&quot;: &quot;recipients&quot;,
      &quot;value&quot;: &quot;254722852258&quot;
    },
    {
      &quot;name&quot;: &quot;template_id&quot;,
      &quot;value&quot;: &quot;7&quot;
    },
    {
      &quot;name&quot;: &quot;goal_name&quot;,
      &quot;value&quot;: &quot;Rent&quot;
    },
    {
      &quot;name&quot;: &quot;amount&quot;,
      &quot;value&quot;: &quot;10000&quot;
    },
    {
      &quot;name&quot;: &quot;deposit&quot;,
      &quot;value&quot;: &quot;2500&quot;
    },
    {
      &quot;name&quot;: &quot;till_number&quot;,
      &quot;value&quot;: &quot;695071&quot;
    },
    {
      &quot;name&quot;: &quot;end_date&quot;,
      &quot;value&quot;: &quot;74-32-2085&quot;
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
